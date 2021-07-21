#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket::State;
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};
use serde::Serialize;


use std::sync::atomic::{AtomicUsize, Ordering};

struct HitCount {
    count: Mutex<AtomicUsize>
}

#[get("/")]
fn index(hit_count: State<HitCount>) -> Template {
    let count = hit_count.count.lock().unwrap().load(Ordering::Relaxed);
    let context: HashMap<&str, usize> = [("count", count)]
        .iter().cloned().collect();
    Template::render("index", &context)
}

#[derive(Serialize)]
struct HitCountRes {
    count: usize
}

#[get("/bump-count")]
fn bump_count(hit_count: State<HitCount>) -> Json<HitCountRes> {
    *hit_count.count.lock().unwrap().get_mut() += 1;
    let count = hit_count.count.lock().unwrap().load(Ordering::Relaxed);
    Json(HitCountRes {
        count
    })
    
}

fn main() {
    rocket::ignite()
        .mount("/public", StaticFiles::from("out"))
        .mount("/", routes![index, bump_count])
        .manage(HitCount { count: Mutex::new(AtomicUsize::new(6)) })
        .attach(Template::fairing())
        .launch();
}