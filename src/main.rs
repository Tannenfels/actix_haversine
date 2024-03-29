mod haversine;

use actix_web::{web, App, HttpServer, post};
use haversine::{Point};
use serde::Deserialize;
use crate::haversine::haversine_formula;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(haversine_internal)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
#[post("/hv_formula")]
async fn haversine_internal(req_body: web::Json<Coordinates>) -> Result<String, actix_web::http::Error> {
    let start_point = Point{
        lat: if req_body.lat_start != 0.0 {req_body.lat_start} else {1.0},
        lon: if req_body.lon_start != 0.0 {req_body.lon_start} else {1.0}
    };
    let end_point = Point{
        lat: if req_body.lat_end != 0.0 {req_body.lat_end} else {1.0},
        lon: if req_body.lon_end != 0.0 {req_body.lon_end} else {1.0}
    };


    let distance = haversine_formula(start_point, end_point);
    return Ok(format!("Distance in meters is: {}", distance));
}

#[derive(Deserialize)]
struct Coordinates {
    lat_start: f64,
    lon_start: f64,
    lat_end: f64,
    lon_end: f64,
}
