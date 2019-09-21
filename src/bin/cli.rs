#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::io;
use std::io::{Write};
use std::net::IpAddr;

#[path = "../models.rs"]
mod models;
#[path = "../schema.rs"]
mod schema;

use models::{NewIpAddress};

fn main() {
    let conn = establish_connection();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        process_command(&command, &conn);
    }
}

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).unwrap()
}

fn process_command(command: &str, conn: &MysqlConnection) {
    let v: Vec<&str> = command.split_whitespace().collect();

    if v.len() == 0 { return }

    let cmd = v[0].to_ascii_uppercase();
    if cmd == "ADD" {
        add_ip(conn, v[1]);
    } else if cmd == "REMOVE" {
        remove_ip(conn, v[1]);
    }
    
    if cmd == "EXIT" || cmd == "QUIT" {
        std::process::exit(0);
    }
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
