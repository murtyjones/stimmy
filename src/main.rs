#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket_contrib::{serve::StaticFiles, templates::Template};


#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("name", "Jonathan")]
        .iter().cloned().collect();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
    .mount("/public", StaticFiles::from("out"))
    .mount("/", routes![index])
    .attach(Template::fairing())
    .launch();
}