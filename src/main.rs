#![feature(proc_macro_hygiene, decl_macro, let_chains)]

#[macro_use] extern crate rocket;


use std::time::Duration;
use std::{collections::HashMap, thread::sleep};
use std::sync::Mutex;
use rocket::figment::value::Value;
use rocket::form::Form;
use rocket::http::ContentType;
use rocket::response::content;
use rocket::fs::{FileServer, relative};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rand::Rng;
use rocket::State;
use rocket::tokio::runtime::Handle;
use rocket_dyn_templates::handlebars::{JsonValue, Renderable};
use rocket_dyn_templates::Template;
use rocket_dyn_templates::handlebars::handlebars_helper;
use std::sync::atomic::{AtomicUsize, Ordering};

#[macro_use] mod context;

struct HitCount {
    count: Mutex<AtomicUsize>
}

#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
struct Profile {
    username: String,
    upshot: String,
    sun_sign: String,
    industry: String,
    description: String,
    profile_pic_b64: String,
}

#[derive(Serialize, Deserialize)]
struct Profiles(Mutex<Vec<Profile>>);

#[derive(Serialize, Deserialize)]
struct Response {
    success: bool
}

const SUN_SIGNS: [&'static str; 12] = [
    "Capricorn",
    "Aquarius",
    "Pisces",
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius"
];

#[get("/optimistic-ui")]
fn index(hit_count: &State<HitCount>) -> Template {
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
    Template::render("create_profile_page", context! {})
}

#[get("/profiles")]
fn list_profiles(profiles: &State<Profiles>) -> Template {
    random_short_sleep();
    let profiles: Vec<Profile> = profiles.0.lock().unwrap().iter().cloned().collect();
    Template::render("list_profiles_page", context! {
        profiles
    })
}

#[get("/profiles/<username>")]
fn show_profile(username: String, profiles: &State<Profiles>) -> Template {
    random_short_sleep();
    let user: Option<Profile> = profiles.0.lock().unwrap().iter()
        .find(|e| e.username == username).cloned();
    Template::render("show_profile_page", context! {
        user
    })
}

#[get("/profiles/<username>/description")]
fn description(username: String, profiles: &State<Profiles>) -> Template {
    // Make this one take extra long so that a load spinner fires for a while
    random_short_sleep();
    random_short_sleep();
    random_short_sleep();
    random_short_sleep();
    random_short_sleep();
    let user: Option<Profile> = profiles.0.lock().unwrap().iter()
        .find(|e| e.username == username).cloned();
    Template::render("description", context! {
        description: user.unwrap().description
    })
}

#[get("/profiles/<username>/edit")]
fn edit_profile(username: String, profiles: &State<Profiles>) -> Template {
    random_short_sleep();
    let user: Option<Profile> = profiles.0.lock().unwrap().iter()
        .find(|e| e.username == username).cloned();
    Template::render("edit_profile_page", context! {
        user: user.unwrap()
    })
}

#[post("/profiles/new", data = "<user_form>")]
fn create_profile(user_form: Json<Profile>, profiles: &State<Profiles>) -> Json<Response> {
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
fn check_username_availability(username: String, profiles: &State<Profiles>) -> Json<FieldValidationResponse> {
    random_short_sleep();
    let is_available = profiles.0.lock().unwrap().iter().find(|each| each.username == username).is_none();
    Json(FieldValidationResponse {
        is_valid: is_available
    })
}

#[derive(FromForm, Serialize, Deserialize, Clone)]
struct FindProfile {
    username: Vec<String>
}

#[post("/profiles/find", data = "<form>")]
fn find_profiles(form: Form<FindProfile>, profiles: &State<Profiles>) -> content::Custom<Template> {
    let profiles = profiles.0.lock().unwrap();
    let stream = ContentType::new("text", "vnd.turbo-stream.html");
    content::Custom(stream, Template::render("user_search_results", context! {
        users: profiles.clone()
    }))
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]

enum FilterContext {
    Industry(String),
    Profiles(Vec<Profile>),
    SunSigns(Vec<String>),
}

#[derive(FromForm, Debug)]
struct Filters {
    industry: Vec<String>,
    sun_sign: Vec<String>,
}

#[get("/profiles/filter?<filters..>")]
fn profiles_filter(filters: Filters, profiles: &State<Profiles>) -> Template {
    random_short_sleep();
    let profiles = profiles.0.lock().unwrap();
    let matching_profiles: Vec<Profile> = profiles.iter().cloned()
    .filter(|e| {
        if filters.industry.len() > 0 && !filters.industry.contains(&e.industry) {
            return false;
        }
        if filters.sun_sign.len() > 0 && !filters.sun_sign.contains(&e.sun_sign) {
            return false;
        }
        true
    }).collect();
    Template::render("filter", context! {
        profiles: matching_profiles,
        checked_industries: filters.industry,
        checked_sun_signs: filters.sun_sign,
        all_sun_signs: SUN_SIGNS,
    })
}


#[get("/bump-count")]
fn bump_count(hit_count: &State<HitCount>) -> Json<Response> {
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
    let delay_ms = rng.gen_range(400..=600);
    sleep(Duration::from_millis(delay_ms));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from(relative!("out")))
        .mount("/css", FileServer::from(relative!("css")))
        .mount("/", routes![
            index, bump_count, create_profile_page, list_profiles, create_profile, edit_profile,
            show_profile, check_username_availability, description, find_profiles, profiles_filter
        ])
        .manage(HitCount { count: Mutex::new(AtomicUsize::new(6)) })
        .manage(
            Profiles(
                Mutex::new(vec![Profile {
                    username: "stevejobs".to_string(),
                    upshot: "Co-founder of Apple".to_string(),
                    sun_sign: "Pisces".to_string(),
                    industry: "tech".to_string(),
                    description: "A college dropout, Steve Jobs, went on to be regarded as the Father of the Digital World. The founder/co-founder of Apple Inc, Pixar Animation Studios and NeXT Inc was a passionate visionary who was responsible for the development of iMac, iPod, iTunes, iPad and the iPhone which ushered in a new era in the computer, music and film industries.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/steve_b64").to_string(),
                }, Profile {
                    username: "jeffbezos".to_string(),
                    sun_sign: "Capricorn".to_string(),
                    industry: "tech".to_string(),
                    upshot: "Founder of Amazon.com".to_string(),
                    description: "The founder and CEO of the multi-national technology company Amazon, Jeff Bezos is the wealthiest man in the world. Jeff, who left his lucrative job at an investment firm to fulfil his entrepreneurial ambition, also owns the newspaper, The Washington Post, and its affiliate publications along with a spaceflight company, Blue Origin.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/jeff_b64").to_string(),
                }, Profile {
                    username: "billgates".to_string(),
                    sun_sign: "Scorpio".to_string(),
                    industry: "tech".to_string(),
                    upshot: "Co-Founder of Microsoft".to_string(),
                    description: "Leading American technologist, business leader and philanthropist, Bill Gates is the co-founder of the world’s largest software company, Microsoft. His passion for computers made him one of the richest in the world and through his charity foundation, Bill & Melinda Gates Foundation, he and his ex-wife, Melinda, use this money generously to help people world over live a better life.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/bill_b64").to_string(),
                }, Profile {
                    username: "markzuckerberg".to_string(),
                    sun_sign: "Taurus".to_string(),
                    industry: "tech".to_string(),
                    upshot: "Chief Executive Officer of Facebook".to_string(),
                    description: "The co-founder and CEO of the popular social networking website, Facebook, Mark Zuckerberg is also amongst the richest men in the world. Fascinated by computer since his early life, Zuckerberg co-created TheFacebook while still in college and later even dropped out to devote more time to it. At 23, he became the youngest self-made billionaire in history at that time.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/mark_b64").to_string(),
                }, Profile {
                    username: "warrenbuffett".to_string(),
                    sun_sign: "Virgo".to_string(),
                    industry: "finance".to_string(),
                    upshot: "Business Magnate, Investor, Philanthropist".to_string(),
                    description: "American investor, business tycoon and philanthropist Warren Buffett is considered one of the most successful investors in the world by the media. The chairman and largest shareholder of the firm, Berkshire Hathaway, he is often called as the ‘Oracle’ or ‘Sage’ of Omaha. Notably, he has pledged to give away a sizable portion of his wealth to philanthropic causes.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/warren_b64").to_string(),
                }, Profile {
                    username: "sundarpichai".to_string(),
                    sun_sign: "Cancer".to_string(),
                    industry: "tech".to_string(),
                    upshot: "CEO of Google Inc".to_string(),
                    description: "Sundar Pichai is the CEO of Google and its parent company Alphabet Inc. An alumnus of IIT, Stanford and Wharton, Pichai has come a long way, from innovating Google’s products such as Chrome, Google Drive and Google Apps to leading the software giant and being one of the most sought-after names in the tech industry.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/sundar_b64").to_string(),
                }, Profile {
                    username: "jackma".to_string(),
                    sun_sign: "Virgo".to_string(),
                    industry: "e-commerce".to_string(),
                    upshot: "Chinese business magnate".to_string(),
                    description: "Jack Ma’s journey from being a tourist guide to establishing the Chinese e-commerce conglomerate Alibaba is inspirational. Initially rejected by various companies, he is the second-wealthiest person in China as of 2020. After quitting his post of executive chairman of Alibaba in 2019, he focused on philanthropy and environmental causes.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/jack_b64").to_string(),
                }, Profile {
                    username: "larrypage".to_string(),
                    sun_sign: "Aries".to_string(),
                    industry: "tech".to_string(),
                    upshot: "Co-founder of Google".to_string(),
                    description: "Larry Page is an American Internet entrepreneur and computer scientist. As one of the co-founders of the multinational technology company Google, Larry Page effectively changed the way the world functions today as Google is a synonym for a search engine in many parts of the world. Unsurprisingly, he became a billionaire after co-founding Google.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/larry_b64").to_string(),
                }, Profile {
                    username: "satyanadella".to_string(),
                    sun_sign: "Leo".to_string(),
                    industry: "tech".to_string(),
                    upshot: "Chairman & CEO of Microsoft".to_string(),
                    description: "Satya Nadella is an Indian-American business executive, currently serving as the chief executive officer of Microsoft. Since he became CEO, the company has seen a 27% annual growth rate. Born in India to a Telugu-speaking family, Nadella serves as an inspiration to millions of Indians who nurse 'The American Dream'.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/satya_b64").to_string(),
                }, Profile {
                    username: "robertkraft".to_string(),
                    sun_sign: "Gemini".to_string(),
                    industry: "sports".to_string(),
                    upshot: "Chief Executive Officer of the New England Patriots".to_string(),
                    description: "Robert Kraft is an American businessman and the chief executive officer and chairman of the popular diversified holding company, The Kraft Group. A sports enthusiast, Kraft owns the Gillette Stadium, NFL's New England Patriots, and MLS' New England Revolution. Also a well-known philanthropist, Robert Kraft has donated millions of dollars to various causes, such as healthcare, education, and youth sports.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/satya_b64").to_string(),
                }, Profile {
                    username: "andrewyang".to_string(),
                    sun_sign: "Capricorn".to_string(),
                    industry: "unknown".to_string(),
                    upshot: "Entrepreneur".to_string(),
                    description: "This guy hasn't really done all that much and seems to be famous mostly for running for president".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/andrew_b64").to_string(),
                }, Profile {
                    username: "timcook".to_string(),
                    sun_sign: "Scorpio".to_string(),
                    industry: "tech".to_string(),
                    upshot: "Chief Executive Officer of Apple".to_string(),
                    description: "Since taking over as the CEO of Apple, Inc. in 2011, till 2020, Tim Cook has, through his dedication, doubled its profits. He was the first Fortune 500 CEO to “come out” as gay, in 2014. A devoted philanthropist, Tim intends to donate most of his stocks to charity.".to_string(),
                    profile_pic_b64: include_str!("../images/ceos/tim_b64").to_string(),
                }])
            )
            
        )
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("contains", Box::new(contains));
        }))
}

use rocket_dyn_templates::handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};

fn contains<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> HelperResult {
    // param 0 should be an array
    // param 1 should be a string
    // param 0 should contain param 1
    let array = h.param(0);
    if array.is_none() {
        return Err(RenderError::new("No array given"));
    }
    let array = array.unwrap().value().as_array();
    if array.is_none() {
        return Err(RenderError::new("First param should be an array"));
    }
    let array = array.unwrap();
    let needle = h.param(1);
    if needle.is_none() {
        return Err(RenderError::new("No needle given"));
    }
    let needle = needle.unwrap().value();
    if array.contains(needle) {
        return h.template()
        .map(|t| t.render(r, ctx, rc, out))
        .unwrap_or(Ok(()));
    } else {
        return h.inverse()
        .map(|t| t.render(r, ctx, rc, out))
        .unwrap_or(Ok(()));
    }
}