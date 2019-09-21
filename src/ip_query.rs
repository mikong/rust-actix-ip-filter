use diesel::prelude::*;
use super::schema;

pub fn ip_exists(conn: &MysqlConnection, ip_str: &str) -> bool {
    use schema::ip_addresses::dsl::*;

    diesel::select(diesel::dsl::exists(ip_addresses.filter(ip.eq(ip_str))))
        .get_result(conn)
        .expect("Error checking existence")
}
