table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    servers (id) {
        id -> Integer,
        name -> Varchar,
        location -> Varchar,
        ip -> Varchar,
        public_key -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    vpn_keys (id) {
        id -> Integer,
        user_id -> Integer,
        server_id -> Nullable<Integer>,
        private_key -> Varchar,
        public_key -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    connections (id) {
        id -> Integer,
        user_id -> Integer,
        server_id -> Integer,
        status -> Varchar,
        created_at -> Timestamp,
    }
}