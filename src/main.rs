#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};

use std::net::IpAddr;

mod models;
mod schema;

struct AppState {
    ips: Vec<IpAddr>,
}

fn index(data: web::Data<AppState>) -> String {
    match &data.ips.first() {
        Some(&ip) => format!("First IP: {}", ip),
        None => format!("No IPs found"),
    }
}

fn main() {
    HttpServer::new(|| {
        let ips = vec!["127.0.0.1".parse().unwrap()];
        App::new()
            .data(AppState {
                ips: ips
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap()
}
