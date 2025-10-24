use crate::schema::servers;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "servers"]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub ip: String,
    pub public_key: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "servers"]
pub struct NewServer<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub ip: &'a str,
    pub public_key: &'a str,
}