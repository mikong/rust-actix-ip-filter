use chrono::NaiveDateTime;
use super::schema::ip_addresses;

#[derive(Queryable)]
pub struct IpAddress {
    pub id: i32,
    pub ip: String,
    pub ts: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="ip_addresses"]
pub struct NewIpAddress<'a> {
    pub ip: &'a str,
}
