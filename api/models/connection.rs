use crate::schema::connections;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[belongs_to(Server)]
#[table_name = "connections"]
pub struct Connection {
    pub id: i32,
    pub user_id: i32,
    pub server_id: i32,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "connections"]
pub struct NewConnection {
    pub user_id: i32,
    pub server_id: i32,
    pub status: String,
}