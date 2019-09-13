#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::net::IpAddr;

mod models;
mod schema;

use models::*;

struct AppState {
    connection: MysqlConnection,
}

fn index(data: web::Data<AppState>) -> String {
    use schema::ip_addresses::dsl::*;

    let results = ip_addresses
        .load::<IpAddress>(&data.connection)
        .expect("Error loading IP addresses");

    for ip_address in results {
        println!("{}", ip_address.ip);
    }

    format!("No IPs found")
}

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connectiong to {}", database_url))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                connection: establish_connection(),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap()
}
