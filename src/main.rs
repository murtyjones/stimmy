#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::time::Duration;
use std::{collections::HashMap, thread::sleep};
use std::sync::Mutex;
use rocket::http::ContentType;
use rocket::response::content::Content;

use rand::Rng;
use rocket::State;
use rocket::request::Form;
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};
use serde::{Serialize, Deserialize};


use std::sync::atomic::{AtomicUsize, Ordering};

struct HitCount {
    count: Mutex<AtomicUsize>
}

#[derive(FromForm, Serialize, Deserialize, Clone)]
struct Profile {
    username: String,
    description: String,
    profile_pic_b64: String,
}

#[derive(Serialize, Deserialize)]
struct Profiles(Mutex<Vec<Profile>>);

#[derive(Serialize, Deserialize)]
struct Response {
    success: bool
}

#[get("/optimistic-ui")]
fn index(hit_count: State<HitCount>) -> Template {
    random_short_sleep();
    let count = hit_count.count.lock().unwrap().load(Ordering::Relaxed);
    let context: HashMap<&str, usize> = [("count", count)]
        .iter().cloned().collect();
    Template::render("index", &context)
}

// #[get("/profiles/new")]
#[get("/")]
fn create_profile_page() -> Template {
    random_short_sleep();
    let context: HashMap<&str, usize> = [].iter().cloned().collect();
    Template::render("create_profile_page", &context)
}

#[get("/profiles")]
fn list_profiles(profiles: State<Profiles>) -> Template {
    random_short_sleep();
    let profiles: Vec<Profile> = profiles.0.lock().unwrap().iter().cloned().collect();
    let context: HashMap<&str, Vec<Profile>> = [("profiles", profiles)]
        .iter().cloned().collect();
    Template::render("list_profiles_page", &context)
}

#[get("/profiles/<username>")]
fn show_profile(username: String, profiles: State<Profiles>) -> Template {
    random_short_sleep();
    let user: Option<Profile> = profiles.0.lock().unwrap().iter()
        .find(|e| e.username == username).cloned();
    let mut context: HashMap<&str, Profile> = HashMap::new();
    context.insert("user", user.unwrap());
    Template::render("show_profile_page", &context)
}

#[get("/profiles/<username>/description")]
fn description(username: String, profiles: State<Profiles>) -> Template {
    // Make this one take extra long so that a load spinner fires for a while
    random_short_sleep();
    random_short_sleep();
    random_short_sleep();
    random_short_sleep();
    random_short_sleep();
    let user: Option<Profile> = profiles.0.lock().unwrap().iter()
        .find(|e| e.username == username).cloned();
    let mut context: HashMap<&str, String> = HashMap::new();
    context.insert("description", user.unwrap().description);
    Template::render("description", &context)
}

#[get("/profiles/<username>/edit")]
fn edit_profile(username: String, profiles: State<Profiles>) -> Template {
    random_short_sleep();
    let user: Option<Profile> = profiles.0.lock().unwrap().iter()
        .find(|e| e.username == username).cloned();
    let mut context: HashMap<&str, Profile> = HashMap::new();
    context.insert("user", user.unwrap());
    Template::render("edit_profile_page", &context)
}

#[post("/profiles/new", data = "<user_form>")]
fn create_profile(user_form: Json<Profile>, profiles: State<Profiles>) -> Json<Response> {
    random_short_sleep();
    let mut profiles = profiles.0.lock().unwrap();
    profiles.push(user_form.0);
    Json(Response {
        success: true
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FieldValidationResponse {
    is_valid: bool
}

#[get("/username-availability/<username>")]
fn check_username_availability(username: String, profiles: State<Profiles>) -> Json<FieldValidationResponse> {
    random_short_sleep();
    let is_available = profiles.0.lock().unwrap().iter().find(|each| each.username == username).is_none();
    Json(FieldValidationResponse {
        is_valid: is_available
    })
}

#[derive(FromForm, Serialize, Deserialize, Clone)]
struct FindProfile {
    username: String
}

#[post("/profiles/find", data = "<form>")]
fn find_profiles(form: Form<FindProfile>, profiles: State<Profiles>) -> Content<Template> {
    let profiles = profiles.0.lock().unwrap();
    let mut context: HashMap<&str, Vec<Profile>> = HashMap::new();
    context.insert("users", profiles.clone());
    let stream = ContentType::new("text", "vnd.turbo-stream.html");
    Content(stream, Template::render("user_search_results", &context))
}


#[get("/bump-count")]
fn bump_count(hit_count: State<HitCount>) -> Json<Response> {
    random_short_sleep();
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

// A delay of 250-350ms to mimick the normal delay from the server
fn random_short_sleep() {
    let mut rng = rand::thread_rng();
    let delay_ms = rng.gen_range(250..=350);
    sleep(Duration::from_millis(delay_ms));
}

fn main() {
    rocket::ignite()
        .mount("/public", StaticFiles::from("out"))
        .mount("/css", StaticFiles::from("css"))
        .mount("/", routes![
            index, bump_count, create_profile_page, list_profiles, create_profile, edit_profile,
            show_profile, check_username_availability, description, find_profiles
        ])
        .manage(HitCount { count: Mutex::new(AtomicUsize::new(6)) })
        .manage(
            Profiles(
                Mutex::new(vec![Profile {
                    username: "murtyjones".to_string(),
                    description: "A really good looking and nice guy".to_string(),
                    profile_pic_b64: include_str!("../marty_pic_b64").to_string(),
                }, Profile {
                    username: "jeffbezos".to_string(),
                    description: "Billionaire extraordinaire".to_string(),
                    profile_pic_b64: include_str!("../jeff_pic_b64").to_string(),
                }])
            )
            
        )
        .attach(Template::fairing())
        .launch();
}