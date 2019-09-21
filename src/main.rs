use actix_files as fs;
use actix_web::{web, App, Error, HttpServer, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::Pool};
use dotenv::dotenv;
use std::env;
use std::net::{SocketAddr};

mod actor;
mod html_list;

use actor::ws::WsActor;
use actix_ip_filter::models::{IpAddress};
use actix_ip_filter::schema;
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
    if let Some(remote) = req.connection_info().remote() {
        if let Ok(socket_addr) = remote.parse::<SocketAddr>() {
            let data: web::Data<AppState> = req.get_app_data().unwrap();
            let conn = &data.pool.get().unwrap();
            let ip_str = &socket_addr.ip().to_string();
            if ip_exists(conn, ip_str) {
                let resp = ws::start(WsActor {}, &req, stream);
                println!("{:?}", resp);
                return resp;
            }
        }
    }
    Ok(HttpResponse::Forbidden().finish())
}

fn ip_exists(conn: &MysqlConnection, ip_str: &str) -> bool {
    use schema::ip_addresses::dsl::*;

    diesel::select(diesel::dsl::exists(ip_addresses.filter(ip.eq(ip_str))))
        .get_result(conn)
        .expect("Error checking existence")
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
    let address = "127.0.0.1:8088";
    let server = HttpServer::new(|| {
        let pool = establish_connection();
        App::new()
            .service(
                web::resource("/ws/")
                    .data(AppState {
                        pool: pool.clone(),
                    })
                    .route(web::get().to(ws_index)))
            .service(fs::Files::new("/client/", "static/").index_file("index.html"))
            .service(
                web::scope("/")
                    .data(AppState {
                        pool: pool.clone(),
                    })
                    .route("/ips", web::get().to(index)))
    })
    .bind(address)
    .unwrap();

    println!("Listening on:");
    println!("http://{}", address);
    println!("ws://{}/ws/", address);

    server.run().unwrap();    
}
