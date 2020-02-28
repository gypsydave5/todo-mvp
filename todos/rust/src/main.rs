use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use lazy_static::lazy_static;
use serde::Serialize;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tera::{Context, Tera};
use uuid::Uuid;

type HandlerResult = Result<Response<Body>, anyhow::Error>;

// Stand-in 404 response
static NOTFOUND: &[u8] = b"Oops! Not Found";

// Todo type

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

type Todos = Arc<RwLock<Vec<Todo>>>;

// Initialize Tera template and todo state
// This app only has one file in one directory, but this way it's ready for you to organize
lazy_static! {
    pub static ref TERA: Tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    pub static ref TODOS: Todos = Arc::new(RwLock::new(Vec::new()));
}

// State manipulation

fn add_todo(t: Todo) {
    let todos = Arc::clone(&TODOS);
    let mut lock = todos.write().unwrap();
    lock.push(t);
}

fn remove_todo(id: Uuid) {
    let todos = Arc::clone(&TODOS);
    let mut lock = todos.write().unwrap();
    // find the index
    let mut idx = lock.len();
    for (i, todo) in lock.iter().enumerate() {
        if todo.id == id {
            idx = i;
        }
    }
    // remove that element if found
    if idx < lock.len() {
        lock.remove(idx);
    }
}

fn toggle_todo(id: Uuid) {
    let todos = Arc::clone(&TODOS);
    let mut lock = todos.write().unwrap();
    for todo in &mut *lock {
        if todo.id == id {
            todo.done = !todo.done;
        }
    }
}

// Routes

async fn four_oh_four() -> HandlerResult {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(NOTFOUND))?)
}

async fn index() -> HandlerResult {
    // Set up index page template rendering context
    let mut ctx = Context::new();
    let todos = Arc::clone(&TODOS);
    let lock = todos.read().unwrap();
    ctx.insert("todos", &*lock);
    ctx.insert("todosLen", &(*lock).len());
    // Render the index template with the context
    let body = Body::from(TERA.render("index.html", &ctx)?.to_string());
    Ok(Response::new(body))
}

async fn stylesheet() -> HandlerResult {
    let body = Body::from(include_str!("resource/todo.css"));
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/css")
        .body(body)?)
}

async fn image(path_str: &str) -> HandlerResult {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    let ext = path_buf.extension().unwrap().to_str().unwrap();

    match ext {
        // TODO - PNG
        "svg" => {
            // build the response
            let body = {
                let xml = match file_name {
                    "check.svg" => include_str!("resource/check.svg"),
                    "plus.svg" => include_str!("resource/plus.svg"),
                    "trashcan.svg" => include_str!("resource/trashcan.svg"),
                    "x.svg" => include_str!("resource/x.svg"),
                    _ => "",
                };
                Body::from(xml)
            };
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/svg+xml")
                .body(body)?)
        }
        _ => four_oh_four().await,
    }
}

async fn redirect_home() -> HandlerResult {
    Ok(Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .body(Body::from(""))?)
}

async fn add_todo_handler(req: Request<Body>) -> HandlerResult {
    let body = hyper::body::to_bytes(req).await?;
    let str_body = String::from_utf8(body.to_vec()).unwrap();
    let words: Vec<&str> = str_body.split('=').collect();
    add_todo(Todo::new(words[1]));
    redirect_home().await
}

async fn remove_todo_handler(req: Request<Body>) -> HandlerResult {
    let body = hyper::body::to_bytes(req).await?;
    let str_body = String::from_utf8(body.to_vec()).unwrap();
    let words: Vec<&str> = str_body.split('=').collect();
    remove_todo(Uuid::parse_str(words[1]).unwrap());
    redirect_home().await
}

async fn toggle_todo_handler(req: Request<Body>) -> HandlerResult {
    let body = hyper::body::to_bytes(req).await?;
    let str_body = String::from_utf8(body.to_vec()).unwrap();
    let words: Vec<&str> = str_body.split('=').collect();
    toggle_todo(Uuid::parse_str(words[1]).unwrap());
    redirect_home().await
}

async fn router(req: Request<Body>) -> HandlerResult {
    // pattern match for both the method and the path of the request
    match (req.method(), req.uri().path()) {
        // GET handlers
        // Index page handler
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index().await,
        // Style handler
        (&Method::GET, "/static/todo.css") => stylesheet().await,
        // Image handler
        (&Method::GET, path_str) => image(path_str).await,
        // POST handlers
        (&Method::POST, "/done") => toggle_todo_handler(req).await,
        (&Method::POST, "/not-done") => toggle_todo_handler(req).await,
        (&Method::POST, "/delete") => remove_todo_handler(req).await,
        (&Method::POST, "/") => add_todo_handler(req).await,
        // Anything else handler
        _ => four_oh_four().await,
    }
}

// Entrypoint

#[tokio::main]
async fn main() {
    // .parse() parses to a std::net::SocketAddr
    let addr = "127.0.0.1:3000".parse().unwrap();
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, anyhow::Error>(service_fn(router)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Serving {} on {}", env!("CARGO_PKG_NAME"), addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
