table! {
    session_tb (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    user_tb (email) {
        email -> Varchar,
        passwd -> Varchar,
        username -> Varchar,
        nickname -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(session_tb, user_tb,);
