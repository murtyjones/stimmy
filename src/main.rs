#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::time::Duration;
use std::{collections::HashMap, thread::sleep};
use std::sync::Mutex;

use rand::Rng;
use rocket::State;
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};
use serde::Serialize;


use std::sync::atomic::{AtomicUsize, Ordering};

struct HitCount {
    count: Mutex<AtomicUsize>
}

#[get("/optimistic-ui")]
fn index(hit_count: State<HitCount>) -> Template {
    let count = hit_count.count.lock().unwrap().load(Ordering::Relaxed);
    let context: HashMap<&str, usize> = [("count", count)]
        .iter().cloned().collect();
    Template::render("index", &context)
}

// #[get("/profile/new")]
#[get("/")]
fn create_profile_page() -> Template {
    let context: HashMap<&str, usize> = [].iter().cloned().collect();
    Template::render("create_profile_page", &context)
}

#[derive(Serialize)]
struct Response {
    success: bool
}

#[get("/bump-count")]
fn bump_count(hit_count: State<HitCount>) -> Json<Response> {
    // We add a delay to mimick the normal delay from the server
    sleep(Duration::from_millis(250));
    let mut rng = rand::thread_rng();
    // Should succeed 80% of the time
    let should_succeed = rng.gen_range(0..5) != 4;
    if should_succeed {
        *hit_count.count.lock().unwrap().get_mut() += 1;
        return Json(Response {
            success: true
        });
    }
    Json(Response {
        success: false
    })
    
}

fn main() {
    rocket::ignite()
        .mount("/public", StaticFiles::from("out"))
        .mount("/css", StaticFiles::from("css"))
        .mount("/", routes![index, bump_count, create_profile_page])
        .manage(HitCount { count: Mutex::new(AtomicUsize::new(6)) })
        .attach(Template::fairing())
        .launch();
}