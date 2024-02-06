use std::sync::atomic::{AtomicUsize};
use lazy_static::lazy_static;
use rocket::{get, launch, post, routes};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::serde::uuid::*;

lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
}


#[derive(Debug, Deserialize)]
struct Message {
    msg: String,
}

#[derive(Debug, Serialize)]
struct LoggedMessage {
    uuid: Uuid,
    msg: String,
}

#[post("/", format = "json", data = "<message>")]
async fn index_post(message: Json<Message>) -> String {
    let uuid = Uuid::new_v4();

    let logged_message = LoggedMessage {
        uuid,
        msg: message.into_inner().msg,
    };

    reqwest::Client::new()
        .post("http://localhost:8001/log")
        .json(&logged_message)
        .send()
        .await
        .expect("Failed to send message to logging-service");

    uuid.to_string()
}

#[get("/")]
async fn index_get() -> String {
    let logging_service_response = reqwest::get("http://localhost:8001/logs")
        .await
        .expect("Failed to get logs")
        .text()
        .await
        .expect("Failed to read logs response");

    let messages_service_response = reqwest::get("http://localhost:8002/message")
        .await
        .expect("Failed to get message")
        .text()
        .await
        .expect("Failed to read message response");

    format!("{} {}", logging_service_response, messages_service_response)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8000)))
        .mount("/", routes![index_post, index_get])
}