#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Awesome {
    message: String,
}

const LENGTH: usize = 2;

#[derive(Serialize)]
struct Awesomes {
    awesomes: [&'static str; LENGTH]
}

const AWESOMES: [&str; LENGTH] = ["You are awesome! 🤩", "You are great! 😍"];


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/awesome")]
fn awesome() -> Json<Awesome> {
    Json(Awesome {  message: String::from(AWESOMES[0]) })
}

#[get("/daily")]
fn daily() -> Json<Awesome> {
    Json(Awesome { message: String::from(AWESOMES[1])})
}

#[get("/all")]
fn all() -> Json<Awesomes> {
    Json(Awesomes { awesomes: AWESOMES })
}

fn main() {
    rocket::ignite().mount("/", routes![index, awesome, daily, all]).launch();
}
