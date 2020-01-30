use argonautica::{Hasher, Verifier};

use warp::{
    reject::{custom, not_found},
    Filter, Rejection, Reply,
};

lazy_static::lazy_static! {
pub  static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

pub fn hash_password(password: &str) -> Result<String, Rejection> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            Err
        })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, warp::Error> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            Err
        })
}
