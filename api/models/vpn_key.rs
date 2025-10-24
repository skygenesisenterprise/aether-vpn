use crate::schema::vpn_keys;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[belongs_to(Server, optional)]
#[table_name = "vpn_keys"]
pub struct VpnKey {
    pub id: i32,
    pub user_id: i32,
    pub server_id: Option<i32>,
    pub private_key: String,
    pub public_key: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "vpn_keys"]
pub struct NewVpnKey<'a> {
    pub user_id: i32,
    pub server_id: Option<i32>,
    pub private_key: &'a str,
    pub public_key: &'a str,
}