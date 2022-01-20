#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Awesome {
    message: String,
}

static AWESOME: [&str; 2] = ["You are awesome! 🤩", "You are great! 😍"];

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/awesome")]
fn json() -> Json<Awesome> {
    Json(Awesome {  message: String::from(AWESOME[0]) })
}

#[get("/daily")]
fn daily() -> Json<Awesome> {
    Json(Awesome { message: String::from(AWESOME[1])})
}

fn main() {
    rocket::ignite().mount("/", routes![index, json, daily]).launch();
}
