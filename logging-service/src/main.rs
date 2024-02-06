use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rocket::{get, launch, post, routes};

use rocket::serde::json::Json;lazy_static! {
    static ref MESSAGES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggedMessage {
    uuid: String,
    msg: String,
}

#[post("/log", format = "json", data = "<message>")]
fn log_message(message: Json<LoggedMessage>) {
    let mut messages = MESSAGES.lock().unwrap();
    let LoggedMessage { uuid, msg } = message.into_inner();
    messages.insert(uuid, msg.clone());
    println!("Logged: {}", msg);
}

#[get("/logs")]
fn get_logs() -> String {
    let messages = MESSAGES.lock().unwrap();
    messages.values().cloned().collect::<Vec<String>>().join(", ")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .mount("/", routes![log_message, get_logs])
}
