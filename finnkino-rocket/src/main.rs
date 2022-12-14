#[macro_use]
extern crate rocket;
extern crate libfinnkino_core;

use rocket::http::{ContentType, Status};

use libfinnkino_core::json::Errors as JsonErrors;
use libfinnkino_rocket::get_areas;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/areas")]
async fn areas() -> (Status, (ContentType, String)) {
  match get_areas().await {
    Err(error) => {
      let errors = JsonErrors::from(error);
      match serde_json::to_string(&errors) {
        Err(error) => (
          Status::InternalServerError,
          (ContentType::Text, error.to_string()),
        ),
        Ok(json) => (Status::Ok, (ContentType::JSON, json)),
      }
    }
    Ok(areas) => match serde_json::to_string(&areas) {
      Err(error) => (
        Status::InternalServerError,
        (ContentType::Text, error.to_string()),
      ),
      Ok(json) => (Status::Ok, (ContentType::JSON, json)),
    },
  }
}

#[rocket::main]
async fn main() {
  let result = rocket::build()
    .mount("/", routes![index])
    .mount("/api", routes![areas])
    .launch()
    .await;
  if let Err(error) = result {
    println!("Launch failed! Error: {}", error);
    return;
  }

  // this is reachable only after `Shutdown::notify()` or `Ctrl+C`.
  println!("Rocket: deorbit.");
}
