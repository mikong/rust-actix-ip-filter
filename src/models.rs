use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct IpAddress {
    pub id: i32,
    pub ip: String,
    pub ts: NaiveDateTime,
}
