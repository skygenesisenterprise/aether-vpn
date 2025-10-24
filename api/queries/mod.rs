// Database queries module

use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;

pub fn create_user(conn: &PgConnection, username: &str, email: &str, password_hash: &str) -> Result<User, diesel::result::Error> {
    let new_user = NewUser { username, email, password_hash };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user_by_email(conn: &PgConnection, user_email: &str) -> Result<User, diesel::result::Error> {
    users::table.filter(users::email.eq(user_email)).first(conn)
}

pub fn get_all_users(conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    users::table.load::<User>(conn)
}

pub fn get_user(conn: &PgConnection, user_id: i32) -> Result<User, diesel::result::Error> {
    users::table.find(user_id).first(conn)
}

pub fn create_server(conn: &PgConnection, name: &str, location: &str, ip: &str, public_key: &str) -> Result<Server, diesel::result::Error> {
    let new_server = NewServer { name, location, ip, public_key };
    diesel::insert_into(servers::table)
        .values(&new_server)
        .get_result(conn)
}

pub fn get_all_servers(conn: &PgConnection) -> Result<Vec<Server>, diesel::result::Error> {
    servers::table.load::<Server>(conn)
}

pub fn get_server(conn: &PgConnection, server_id: i32) -> Result<Server, diesel::result::Error> {
    servers::table.find(server_id).first(conn)
}

pub fn get_servers_by_location(conn: &PgConnection, location: &str) -> Result<Vec<Server>, diesel::result::Error> {
    servers::table.filter(servers::location.eq(location)).load::<Server>(conn)
}

pub fn create_vpn_key(conn: &PgConnection, user_id: i32, server_id: Option<i32>, private_key: &str, public_key: &str) -> Result<VpnKey, diesel::result::Error> {
    let new_key = NewVpnKey { user_id, server_id, private_key, public_key };
    diesel::insert_into(vpn_keys::table)
        .values(&new_key)
        .get_result(conn)
}

pub fn get_vpn_keys_for_user(conn: &PgConnection, user_id: i32) -> Result<Vec<VpnKey>, diesel::result::Error> {
    vpn_keys::table.filter(vpn_keys::user_id.eq(user_id)).load::<VpnKey>(conn)
}

pub fn create_connection(conn: &PgConnection, user_id: i32, server_id: i32, status: &str) -> Result<Connection, diesel::result::Error> {
    let new_connection = NewConnection { user_id, server_id, status: status.to_string() };
    diesel::insert_into(connections::table)
        .values(&new_connection)
        .get_result(conn)
}