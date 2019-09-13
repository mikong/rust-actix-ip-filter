#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_files as fs;
use actix_web::{web, App, Error, HttpServer, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::Pool};
use dotenv::dotenv;
use std::env;
use std::net::IpAddr;

mod actor;
mod models;
mod schema;
mod html_list;

use actor::ws::WsActor;
use models::{IpAddress, NewIpAddress};
use html_list::HtmlList;

struct AppState {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

fn index(data: web::Data<AppState>) -> HttpResponse {
    use schema::ip_addresses::dsl::*;

    let conn = &data.pool.get().unwrap();

    let results = ip_addresses
        .load::<IpAddress>(conn)
        .expect("Error loading IP addresses");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(HtmlList::new(results))
}

fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsActor {}, &req, stream);
    println!("{:?}", resp);
    resp
}

fn add(data: web::Data<AppState>) -> String {
    let ip = "127.0.0.1";

    let conn = &data.pool.get().unwrap();
    add_ip(conn, ip);

    "OK".to_string()
}

fn remove(data: web::Data<AppState>) -> String {
    let ip = "127.0.0.1";

    let conn = &data.pool.get().unwrap();
    remove_ip(conn, ip)
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

fn establish_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build connection pool")
}

fn main() {
    HttpServer::new(|| {
        let pool = establish_connection();
        App::new()
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(fs::Files::new("/client/", "static/").index_file("index.html"))
            .service(
                web::scope("/")
                    .data(AppState {
                        pool: pool.clone(),
                    })
                    .route("/ips", web::get().to(index))
                    .route("/add", web::get().to(add))
                    .route("/remove", web::get().to(remove)))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap()
}
