use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Calculation {
    num1: f64,
    operator: char,
    num2: f64,
    result: Option<f64>,
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/calculate", web::post().to(calculate))
            .data(web::JsonConfig::default()) // Configure JSON serialization
    })
    .bind("localhost:8080")?
    .run()
    .await
}

async fn calculate(calc: web::Json<Calculation>) -> Result<web::Json<Calculation>, String> {
    let mut calculation = calc.into_inner();
    match calculation.operator {
        '+' => calculation.result = Some(calculation.num1 + calculation.num2),
        '-' => calculation.result = Some(calculation.num1 - calculation.num2),
        '*' => calculation.result = Some(calculation.num1 * calculation.num2),
        '/' => {
            if calculation.num2 == 0.0 {
                return Err(String::from("Division by zero error"));
            }
            calculation.result = Some(calculation.num1 / calculation.num2);
        }
        _ => return Err(String::from("Invalid operator")),
    }
    Ok(web::Json(calculation))
}
