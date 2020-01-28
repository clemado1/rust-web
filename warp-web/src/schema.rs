table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

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
        hash -> Varchar,
        passwd -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    invitations,
    session_tb,
    user_tb,
    users,
);
