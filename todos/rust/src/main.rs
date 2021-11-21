use anyhow::Result;
use backtrace::Backtrace;
use clap::Parser;
use flate2::{write::ZlibEncoder, Compression};
use futures::{future::FutureExt, Future};
use hyper::http;
use lazy_static::lazy_static;
use serde::Serialize;
use std::{
    cell::RefCell,
    convert::Infallible,
    io::Write,
    panic::AssertUnwindSafe,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tera::Tera;
use uuid::Uuid;

type Request = http::Request<hyper::Body>;
type Response = http::Response<hyper::Body>;

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
struct Todos(Vec<Todo>);

impl Todos {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, todo: Todo) {
        self.0.push(todo);
    }

    pub fn remove(&mut self, id: Uuid) -> Option<Todo> {
        let mut idx = self.0.len();
        for (i, todo) in self.0.iter().enumerate() {
            if todo.id == id {
                idx = i;
            }
        }
        if idx < self.0.len() {
            let ret = self.0.remove(idx);
            Some(ret)
        } else {
            None
        }
    }

    pub fn todos(&self) -> &[Todo] {
        &self.0
    }

    pub fn toggle(&mut self, id: Uuid) {
        for todo in &mut self.0 {
            if todo.id == id {
                todo.done = !todo.done;
            }
        }
    }
}

// Response builders

async fn bytes_handler(body: &[u8], content_type: &str, status_code: http::StatusCode) -> Response {
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

async fn string_handler(body: &str, content_type: &str, status_code: http::StatusCode) -> Response {
    bytes_handler(body.as_bytes(), content_type, status_code).await
}

async fn ok_string_handler(body: &str, content_type: &str) -> Response {
    string_handler(body, content_type, hyper::StatusCode::OK).await
}

async fn html_str_handler(html: &str, status_code: http::StatusCode) -> Response {
    string_handler(html, "text/html", status_code).await
}

async fn ok_html_handler(html: &str) -> Response {
    html_str_handler(html, http::StatusCode::OK).await
}

// Endpoint handlers

async fn four_oh_four() -> Response {
    html_str_handler("<h1>NOT FOUND!</h1>", http::StatusCode::NOT_FOUND).await
}

async fn index(request: Request) -> Response {
    // Set up index page template rendering context
    let mut tera_ctx = tera::Context::new();
    let todos_ctx: Arc<RwLock<Todos>> = Arc::clone(request.extensions().get().unwrap());
    {
        let lock = todos_ctx.read().unwrap();
        let todos = lock.todos();
        let len = todos.len();
        tera_ctx.insert("todos", todos);
        tera_ctx.insert("todosLen", &len);
    }
    let html = TERA.render("index.html", &tera_ctx).unwrap().to_string();
    ok_html_handler(&html).await
}

async fn stylesheet() -> Response {
    let body = include_str!("resource/todo.css");
    ok_string_handler(body, "text/css").await
}

async fn image(path_str: &str) -> Response {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    let ext = match path_buf.extension() {
        Some(e) => e.to_str().unwrap(),
        None => return four_oh_four().await,
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
            ok_string_handler(body, "image/svg+xml").await
        }
        _ => four_oh_four().await,
    }
}

async fn redirect_home() -> Response {
    hyper::Response::builder()
        .status(hyper::StatusCode::SEE_OTHER)
        .header(hyper::header::LOCATION, "/")
        .body(hyper::Body::from(""))
        .unwrap()
}

/// Get the value after the '=' in a request payload
async fn extract_payload<'a>(request: Request) -> String {
    let body = request.into_body();
    let bytes_buf = hyper::body::to_bytes(body).await.unwrap();
    let str_body = String::from_utf8(bytes_buf.to_vec()).unwrap();
    let words: Vec<&str> = str_body.split('=').collect();
    words[1].to_owned()
}

async fn add_todo_handler(request: Request) -> Response {
    let todos_ctx: Arc<RwLock<Todos>> = Arc::clone(request.extensions().get().unwrap());
    let payload = extract_payload(request).await;
    {
        let mut lock = todos_ctx.write().unwrap();
        (*lock).push(Todo::new(&payload));
    }
    redirect_home().await
}

async fn remove_todo_handler(request: Request) -> Response {
    let todos_ctx: Arc<RwLock<Todos>> = Arc::clone(request.extensions().get().unwrap());
    let payload = extract_payload(request).await;
    {
        let mut lock = todos_ctx.write().unwrap();
        (*lock).remove(Uuid::parse_str(&payload).unwrap());
    }
    redirect_home().await
}

async fn toggle_todo_handler(request: Request) -> Response {
    let todos_ctx: Arc<RwLock<Todos>> = Arc::clone(request.extensions().get().unwrap());
    let payload = extract_payload(request).await;
    {
        let mut lock = todos_ctx.write().unwrap();
        (*lock).toggle(Uuid::parse_str(&payload).unwrap());
    }
    redirect_home().await
}

async fn handle(request: Request) -> Response {
    // pattern match for both the method and the path of the request
    match (request.method(), request.uri().path()) {
        // GET handlers
        // Index page handler
        (&hyper::Method::GET, "/") | (&hyper::Method::GET, "/index.html") => index(request).await,
        // Style handler
        (&hyper::Method::GET, "/static/todo.css") => stylesheet().await,
        // Image handler
        (&hyper::Method::GET, path_str) => image(path_str).await,
        // POST handlers
        (&hyper::Method::POST, "/done") => toggle_todo_handler(request).await,
        (&hyper::Method::POST, "/not-done") => toggle_todo_handler(request).await,
        (&hyper::Method::POST, "/delete") => remove_todo_handler(request).await,
        (&hyper::Method::POST, "/") => add_todo_handler(request).await,
        // Anything else handler
        _ => four_oh_four().await,
    }
}

pub async fn serve<C, H, F>(
    addr: std::net::SocketAddr,
    context: Arc<C>,
    handler: H,
) -> hyper::Result<()>
where
    C: 'static + Send + Sync,
    H: 'static + Fn(Request) -> F + Send + Sync,
    F: Future<Output = Response> + Send,
{
    // Create a task local that will store the panic message and backtrace if a panic occurs.
    tokio::task_local! {
        static PANIC_MESSAGE_AND_BACKTRACE: RefCell<Option<(String, Backtrace)>>;
    }
    async fn service<C, H, F>(
        handler: Arc<H>,
        context: Arc<C>,
        mut request: http::Request<hyper::Body>,
    ) -> Result<http::Response<hyper::Body>, Infallible>
    where
        C: Send + Sync + 'static,
        H: Fn(http::Request<hyper::Body>) -> F + Send + Sync + 'static,
        F: Future<Output = http::Response<hyper::Body>> + Send,
    {
        let method = request.method().clone();
        let path = request.uri().path_and_query().unwrap().path().to_owned();
        tracing::info!(path = %path, method = %method, "request");
        request.extensions_mut().insert(context);
        let result = AssertUnwindSafe(handler(request)).catch_unwind().await;
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
    }
    // Install a panic hook that will record the panic message and backtrace if a panic occurs.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|panic_info| {
        let value = (panic_info.to_string(), Backtrace::new());
        PANIC_MESSAGE_AND_BACKTRACE.with(|panic_message_and_backtrace| {
            panic_message_and_backtrace.borrow_mut().replace(value);
        })
    }));
    // Wrap the request handler and context with Arc to allow sharing a reference to it with each task.
    let handler = Arc::new(handler);
    let service = hyper::service::make_service_fn(|_| {
        let handler = handler.clone();
        let context = context.clone();
        async move {
            Ok::<_, Infallible>(hyper::service::service_fn(move |request| {
                let handler = handler.clone();
                let context = context.clone();
                PANIC_MESSAGE_AND_BACKTRACE.scope(RefCell::new(None), async move {
                    service(handler, context, request).await
                })
            }))
        }
    });
    let server = hyper::server::Server::try_bind(&addr)?;
    tracing::info!("🚀 serving at {}", addr);
    server.serve(service).await?;
    std::panic::set_hook(hook);
    Ok(())
}

// Entrypoint
#[tokio::main]
async fn app(args: Args) -> Result<()> {
    tracing_subscriber::fmt::init();

    // .parse() parses to a std::net::SocketAddr
    let addr = std::net::SocketAddr::new(args.address.parse()?, args.port);

    let todos = Todos::new();
    let context = Arc::new(RwLock::new(todos));

    serve(addr, context, handle).await?;

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
        let mut request = hyper::Request::builder()
            .method(http::Method::GET)
            .uri("/nonsense")
            .body(hyper::Body::empty())
            .unwrap();
        let context = Arc::new(RwLock::new(Todos::new()));
        request.extensions_mut().insert(Arc::clone(&context));
        let response = handle(request).await;

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
