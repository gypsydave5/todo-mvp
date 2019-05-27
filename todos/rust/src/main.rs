#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;

use futures::{future, Future, Stream};
use hyper::{
    client::HttpConnector, header, rt, service::service_fn, Body, Client, Method, Request,
    Response, Server, StatusCode,
};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tera::{Context, Tera};
use uuid::Uuid;

// Handler type aliases
type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<Future<Item = Response<Body>, Error = GenericError> + Send>;

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
    pub static ref TERA: Tera = compile_templates!("templates/**/*");
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

fn four_oh_four() -> ResponseFuture {
    let body = Body::from(NOTFOUND);
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body)
            .unwrap(),
    ))
}

fn index() -> ResponseFuture {
    // Set up index page template rendering context
    let mut ctx = Context::new();
    let todos = Arc::clone(&TODOS);
    let lock = todos.read().unwrap();
    ctx.insert("todos", &*lock);
    ctx.insert("todosLen", &(*lock).len());
    // Render the index template with the context
    let body = Body::from(TERA.render("index.html", &ctx).unwrap().to_string());
    Box::new(future::ok(Response::new(body)))
}

fn stylesheet() -> ResponseFuture {
    let body = Body::from(include_str!("resource/todo.css"));
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/css")
            .body(body)
            .unwrap(),
    ))
}

fn image(path_str: &str) -> ResponseFuture {
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
            Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "image/svg+xml")
                    .body(body)
                    .unwrap(),
            ))
        }
        _ => four_oh_four(),
    }
}

fn redirect_home() -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(header::LOCATION, "/")
            .body(Body::from(""))
            .unwrap(),
    ))
}

fn add_todo_handler(req: Request<Body>) -> ResponseFuture {
    Box::new(
        req.into_body()
            .concat2() // concatenate all the chunks in the body
            .from_err() // like try! for Result, but for Futures
            .and_then(|whole_body| {
                let str_body = String::from_utf8(whole_body.to_vec()).unwrap();
                let words: Vec<&str> = str_body.split('=').collect();
                add_todo(Todo::new(words[1]));
                redirect_home()
            }),
    )
}

fn remove_todo_handler(req: Request<Body>) -> ResponseFuture {
    Box::new(
        req.into_body()
            .concat2() // concatenate all the chunks in the body
            .from_err() // like try! for Result, but for Futures
            .and_then(|whole_body| {
                let str_body = String::from_utf8(whole_body.to_vec()).unwrap();
                let words: Vec<&str> = str_body.split('=').collect();
                remove_todo(Uuid::parse_str(words[1]).unwrap());
                redirect_home()
            }),
    )
}

fn toggle_todo_handler(req: Request<Body>) -> ResponseFuture {
    Box::new(
        req.into_body()
            .concat2() // concatenate all the chunks in the body
            .from_err() // like try! for Result, but for Futures
            .and_then(|whole_body| {
                let str_body = String::from_utf8(whole_body.to_vec()).unwrap();
                let words: Vec<&str> = str_body.split('=').collect();
                toggle_todo(Uuid::parse_str(words[1]).unwrap());
                redirect_home()
            }),
    )
}

fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> ResponseFuture {
    // pattern match for both the method and the path of the request
    match (req.method(), req.uri().path()) {
        // GET handlers
        // Index page handler
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index(),
        // Style handler
        (&Method::GET, "/static/todo.css") => stylesheet(),
        // Image handler
        (&Method::GET, path_str) => image(path_str),
        // POST handlers
        (&Method::POST, "/done") => toggle_todo_handler(req),
        (&Method::POST, "/not-done") => toggle_todo_handler(req),
        (&Method::POST, "/delete") => remove_todo_handler(req),
        (&Method::POST, "/") => add_todo_handler(req),
        // Anything else handler
        _ => four_oh_four(),
    }
}

// Entrypoint

fn main() {
    pretty_env_logger::init();

    // .parse() parses to a std::net::SocketAddr
    let addr = "127.0.0.1:3000".parse().unwrap();

    rt::run(future::lazy(move || {
        // create a Client for all Services
        let client = Client::new();

        // define a service containing the router function
        let new_service = move || {
            // Move a clone of Client into the service_fn
            let client = client.clone();
            service_fn(move |req| router(req, &client))
        };

        // Define the server - this is what the future_lazy() we're building will resolve to
        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("Server error: {}", e));

        println!("Listening on http://{}", addr);
        server
    }));
}
