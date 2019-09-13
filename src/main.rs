#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, HttpResponse};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::net::IpAddr;

mod models;
mod schema;
mod html_list;

use models::{IpAddress, NewIpAddress};
use html_list::HtmlList;

struct AppState {
    connection: MysqlConnection,
}

fn index(data: web::Data<AppState>) -> HttpResponse {
    use schema::ip_addresses::dsl::*;

    let results = ip_addresses
        .load::<IpAddress>(&data.connection)
        .expect("Error loading IP addresses");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(HtmlList::new(results))
}

fn add(data: web::Data<AppState>) -> String {
    let ip = "127.0.0.1";

    add_ip(&data.connection, ip);

    "OK".to_string()
}

fn remove(data: web::Data<AppState>) -> String {
    let ip = "127.0.0.1";

    remove_ip(&data.connection, ip)
}

fn add_ip(conn: &MysqlConnection, ip: &str) {
    use schema::ip_addresses;

    let ip_addr: IpAddr = ip.parse().expect("Not a valid IP address");

    let new_ip_address = NewIpAddress {
        ip: &ip_addr.to_string(),
    };

    diesel::insert_into(ip_addresses::table)
        .values(&new_ip_address)
        .execute(conn)
        .expect("Error saving IP address");
}

fn remove_ip(conn: &MysqlConnection, ip_str: &str) -> String {
    use schema::ip_addresses::dsl::*;

    let ip_addr: IpAddr = ip_str.parse().expect("Not a valid IP address");

    let num_deleted = diesel::delete(ip_addresses.filter(ip.eq(ip_addr.to_string())))
        .execute(conn)
        .expect("Error deleting IP addresses");

    format!("Deleted {} IP addresses", num_deleted)
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
            .route("/add", web::get().to(add))
            .route("/remove", web::get().to(remove))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap()
}
