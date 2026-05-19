use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;

mod file_reader;
mod utils;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/run/{day}/{part}", get(run_day_part))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
} 

async fn health() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}

async fn run_day_part(axum::extract::Path((day_number, part)): axum::extract::Path<(String, u8)>) -> impl IntoResponse {
    println!("Received request for day {} part {}", day_number, part);
    let output = match day_number.as_str() {
        "1" => day_01::run(part),
        "2" => day_02::run(part),
        "3" => day_03::run(part),
        "4" => day_04::run(part),
        "5" => day_05::run(part),
        "6" => day_06::run(part),
        "7" => day_07::run(part),
        "8" => day_08::run(part),
        "9" => day_09::run(part),
        // "10" => day_10::run(part),
        // "11" => day_11::run(part),
        // "12" => day_12::run(part),
        _ => return Json(json!({"error": "Invalid day number, the days are 1 to 12"})),
    };

    Json(json!({"output": output}))
}