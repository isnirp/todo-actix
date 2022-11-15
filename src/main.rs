use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, FromRequest};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, json};
// use std::fmt::{self, display};
// use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct Todo {
    name: String,
    description: String
}

impl Todo {
    fn new(name: &str, description: &str) -> Todo {
        Todo {
            name: name.to_string(),
            description: description.to_string()
        }
    }

    fn desc_task(&self) -> String {
        format!("TodoActix - {} {}", self.name, self.description)
    }
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

// to use a struct type as a request the struct must implement a FromRequest trait
async fn add_task(req_todo: web::Json<Todo> ) -> impl Responder {
    // Convert the JSON string back to a Todo.
    // let task: Task = serde_json::json(req_todo).unwrap();
    println!("deserialized = {:?}", req_todo.name);
    HttpResponse::Ok().body("name is {req_todo.name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/add", web::post().to(add_task))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// simple todo crud app with actix
// actix web servers are built around the App instance.
// the app instance registers routes for resources and middleware
// the app also stores application state shared across same scope

// the app scope acts as a namespace for all routes
// the routes within a scope have the same url path /path

// rust ownership
// each value has a variable that's called its owner
// there can only be one owner at a time
// the value is dropped when the owner is out of scope

// collections
// let a: [i32,_] = [1,2,3]
// let v: Vec<i32> = Vec::new(); v.push(1);
// let v = vec![1,2,3];

// String, a collection of UTF-8 encoded bytes
// let s = String::from("hello");

// let hmap = HashMap::new();
// hmap.insert(String::from("key"), 23);