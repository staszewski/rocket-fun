#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
struct Task {
    description: String,
    complete: bool,
}

#[derive(Deserialize, Serialize)]
struct TaskOutput {
    id: String,
    description: String,
    complete: bool,
}

#[post("/todo", data = "<task>")]
fn todo(task: Json<Task>) -> JsonValue {
    let output = TaskOutput {
        id: String::from("123"),
        description: task.0.description,
        complete: task.0.complete,
    };
    json!({
        "status": "200",
        "data": output
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, todo])
        .launch();
}
