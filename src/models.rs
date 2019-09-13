use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct IpAddress {
    pub id: i32,
    pub ip: Vec<u8>,
    pub ts: NaiveDateTime,
}
