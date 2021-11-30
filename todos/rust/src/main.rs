#![feature(type_alias_impl_trait)]

use anyhow::Result;
use backtrace::Backtrace;
use clap::Parser;
use flate2::{write::ZlibEncoder, Compression};
use futures::{future::FutureExt, Future};
use hyper::http;
use lazy_static::lazy_static;
use serde::Serialize;
use std::{
    cell::RefCell, convert::Infallible, io::Write, panic::AssertUnwindSafe, path::PathBuf,
    sync::RwLock,
};
use tera::Tera;
use uuid::Uuid;

type Request = http::Request<hyper::Body>;
type Response = http::Response<hyper::Body>;

#[derive(Debug, Default)]
struct Ctx {
    todos: Todos,
}

/// Todo type
#[derive(Debug, Serialize)]
pub struct Todo {
    done: bool,
    name: String,
    id: Uuid,
}

impl Todo {
    fn new(name: &str) -> Self {
        Self {
            done: false,
            name: String::from(name),
            id: Uuid::new_v4(),
        }
    }
}

// Initialize Tera template and todo state
// This app only has one file in one directory, but this way it's ready for you to organize
lazy_static! {
    pub static ref TERA: Tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Unable to parse templates: {}", e);
            std::process::exit(1);
        }
    };
}

#[derive(Debug, Default)]
struct Todos(RwLock<Vec<Todo>>);

impl Todos {
    pub fn push(&self, todo: Todo) {
        self.0.write().unwrap().push(todo);
    }

    pub fn remove(&self, id: Uuid) -> Option<Todo> {
        let mut todos = self.0.write().unwrap();
        let mut idx = todos.len();
        for (i, todo) in todos.iter().enumerate() {
            if todo.id == id {
                idx = i;
            }
        }
        if idx < todos.len() {
            let ret = todos.remove(idx);
            Some(ret)
        } else {
            None
        }
    }

    pub fn todos_tera_ctx(&self) -> tera::Context {
        let todos = self.0.read().unwrap();
        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("todos", &*todos);
        tera_ctx.insert("todosLen", &todos.len());
        tera_ctx
    }

    pub fn toggle(&self, id: Uuid) {
        for todo in self.0.write().unwrap().iter_mut() {
            if todo.id == id {
                todo.done = !todo.done;
            }
        }
    }
}

// Response builders

fn bytes_handler(body: &[u8], content_type: &str, status_code: http::StatusCode) -> Response {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(body).unwrap();
    let compressed = encoder.finish().unwrap();
    hyper::Response::builder()
        .status(status_code)
        .header(hyper::header::CONTENT_TYPE, content_type)
        .header(hyper::header::CONTENT_ENCODING, "deflate")
        .body(hyper::Body::from(compressed))
        .unwrap()
}

fn string_handler(body: &str, content_type: &str, status_code: http::StatusCode) -> Response {
    bytes_handler(body.as_bytes(), content_type, status_code)
}

fn ok_string_handler(body: &str, content_type: &str) -> Response {
    string_handler(body, content_type, hyper::StatusCode::OK)
}

fn html_str_handler(html: &str, status_code: http::StatusCode) -> Response {
    string_handler(html, "text/html", status_code)
}

fn ok_html_handler(html: &str) -> Response {
    html_str_handler(html, http::StatusCode::OK)
}

// Endpoint handlers

fn four_oh_four() -> Response {
    html_str_handler("<h1>NOT FOUND!</h1>", http::StatusCode::NOT_FOUND)
}

fn index(ctx: &Ctx) -> Response {
    // Set up index page template rendering context
    let tera_ctx = ctx.todos.todos_tera_ctx();
    let html = TERA.render("index.html", &tera_ctx).unwrap();
    ok_html_handler(&html)
}

fn redirect_home(ctx: &Ctx) -> Response {
    // Set up index page template rendering context
    let mut tera_ctx = ctx.todos.todos_tera_ctx();
    tera_ctx.insert("redirect", &true);
    let html = TERA.render("index.html", &tera_ctx).unwrap();
    ok_html_handler(&html)
}

fn stylesheet() -> Response {
    let body = include_str!("resource/todo.css");
    ok_string_handler(body, "text/css")
}

fn image(path_str: &str) -> Response {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    let ext = match path_buf.extension() {
        Some(e) => e.to_str().unwrap(),
        None => return four_oh_four(),
    };

    match ext {
        "svg" => {
            // build the response
            let body = match file_name {
                "check.svg" => include_str!("resource/check.svg"),
                "plus.svg" => include_str!("resource/plus.svg"),
                "trashcan.svg" => include_str!("resource/trashcan.svg"),
                "x.svg" => include_str!("resource/x.svg"),
                _ => "",
            };
            ok_string_handler(body, "image/svg+xml")
        }
        _ => four_oh_four(),
    }
}

/// Get the value after the '=' in a request payload
async fn extract_payload<'a>(request: Request) -> String {
    let body = request.into_body();
    let bytes_buf = hyper::body::to_bytes(body).await.unwrap();
    let str_body = String::from_utf8(bytes_buf.to_vec()).unwrap();
    let words: Vec<&str> = str_body.split('=').collect();
    words[1].to_owned()
}

async fn add_todo_handler(request: Request, ctx: &Ctx) -> Response {
    let payload = extract_payload(request).await;

    ctx.todos.push(Todo::new(&payload));

    redirect_home(ctx)
}

async fn remove_todo_handler(request: Request, ctx: &Ctx) -> Response {
    let payload = extract_payload(request).await;
    let id = Uuid::parse_str(&payload).unwrap();
    ctx.todos.remove(id);

    redirect_home(ctx)
}

async fn toggle_todo_handler(request: Request, ctx: &Ctx) -> Response {
    let payload = extract_payload(request).await;
    let id = Uuid::parse_str(&payload).unwrap();
    ctx.todos.toggle(id);

    redirect_home(ctx)
}

async fn handle(request: Request, ctx: &Ctx) -> Response {
    // pattern match for both the method and the path of the request
    match (request.method(), request.uri().path()) {
        // GET handlers
        // Index page handler
        (&hyper::Method::GET, "/") | (&hyper::Method::GET, "/index.html") => index(ctx),
        // Style handler
        (&hyper::Method::GET, "/static/todo.css") => stylesheet(),
        // Image handler
        (&hyper::Method::GET, path_str) => image(path_str),
        // POST handlers
        (&hyper::Method::POST, "/done") => toggle_todo_handler(request, ctx).await,
        (&hyper::Method::POST, "/not-done") => toggle_todo_handler(request, ctx).await,
        (&hyper::Method::POST, "/delete") => remove_todo_handler(request, ctx).await,
        (&hyper::Method::POST, "/") => add_todo_handler(request, ctx).await,
        // Anything else handler
        _ => four_oh_four(),
    }
}
// Create a task local that will store the panic message and backtrace if a panic occurs.
tokio::task_local! {
    static PANIC_MESSAGE_AND_BACKTRACE: RefCell<Option<(String, Backtrace)>>;
}
async fn serve(addr: std::net::SocketAddr, context: &'static Ctx) -> hyper::Result<()> {
    // Install a panic hook that will record the panic message and backtrace if a panic occurs.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|panic_info| {
        let value = (panic_info.to_string(), Backtrace::new());
        PANIC_MESSAGE_AND_BACKTRACE.with(|panic_message_and_backtrace| {
            panic_message_and_backtrace.borrow_mut().replace(value);
        })
    }));
    let server = hyper::server::Server::try_bind(&addr)?;
    tracing::info!("🚀 serving at {}", addr);
    server.serve(Bogus(context)).await?;
    std::panic::set_hook(hook);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Bogus<'a>(&'a Ctx);

impl<'a, 't> hyper::service::Service<&'t hyper::server::conn::AddrStream> for Bogus<'a> {
    type Error = Infallible;
    type Response = Self;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: &'t hyper::server::conn::AddrStream) -> Self::Future {
        futures::future::ready(Ok(*self))
    }
}
impl<'a> hyper::service::Service<Request> for Bogus<'a> {
    type Response = Response;
    type Error = Infallible;
    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        PANIC_MESSAGE_AND_BACKTRACE.scope(RefCell::new(None), async {
            let method = req.method().clone();
            let path = req.uri().path_and_query().unwrap().path().to_owned();
            tracing::info!(path = %path, method = %method, "request");
            let result = AssertUnwindSafe(handle(req, self.0)).catch_unwind().await;
            let start = std::time::SystemTime::now();
            let response = result.unwrap_or_else(|_| {
                let body = PANIC_MESSAGE_AND_BACKTRACE.with(|panic_message_and_backtrace| {
                    let panic_message_and_backtrace = panic_message_and_backtrace.borrow();
                    let (message, backtrace) = panic_message_and_backtrace.as_ref().unwrap();
                    tracing::error!(
                        method = %method,
                        path = %path,
                        backtrace = ?backtrace,
                        "500"
                    );
                    format!("{}\n{:?}", message, backtrace)
                });
                http::Response::builder()
                    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(hyper::Body::from(body))
                    .unwrap()
            });
            tracing::info!(
                "Response generated in {}μs",
                start.elapsed().unwrap_or_default().as_micros()
            );
            Ok(response)
        })
    }
}

// Entrypoint
#[tokio::main]
async fn app(args: Args) -> Result<()> {
    tracing_subscriber::fmt::init();

    // .parse() parses to a std::net::SocketAddr
    let addr = std::net::SocketAddr::new(args.address.parse()?, args.port);

    let ctx = Box::leak(Default::default());

    serve(addr, ctx).await?;

    Ok(())
}

#[derive(Parser)]
#[clap(version = concat!(env!("CARGO_PKG_VERSION")), about = "Serve a TODO list application.")]
struct Args {
    #[clap(
        short,
        long,
        about = "Address to bind the server to.",
        env,
        default_value = "0.0.0.0"
    )]
    address: String,
    #[clap(short, long, about = "Port to listen on.", env, default_value = "3000")]
    port: u16,
}

fn main() -> Result<()> {
    let args = Args::parse();
    app(args)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use flate2::write::ZlibDecoder;
    use pretty_assertions::assert_eq;
    use select::{document::Document, predicate::Name};
    #[tokio::test]
    async fn test_four_oh_four() {
        let request = hyper::Request::builder()
            .method(http::Method::GET)
            .uri("/nonsense")
            .body(hyper::Body::empty())
            .unwrap();
        let ctx = Default::default();
        let response = handle(request, &ctx).await;

        assert_eq!(response.status(), http::status::StatusCode::NOT_FOUND);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let mut decoder = ZlibDecoder::new(Vec::new());
        decoder.write_all(&body).unwrap();
        let uncompressed = decoder.finish().unwrap();
        let result = String::from_utf8(uncompressed).unwrap();

        let document = Document::from(result.as_str());
        let message = document.find(Name("h1")).next().unwrap().text();
        assert_eq!(message, "NOT FOUND!".to_owned());
    }
}
