use std::env;
use warp::{self, path, Filter};

mod models;

/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /auth`: return a JSON Object of session user.
/// - `POST /auth`: login.
/// - `DELETE /auth`: delete a session user.

pub fn create_connection() -> models::Pool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Faild to create poll")
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    if env::var_os("RUST_LOG").is_none() {
        // also can main=info
        env::set_var("RUST_LOG", "main=debug");
    }
    pretty_env_logger::init();

    let log = warp::log("debug");

    let pool: models::Pool = create_connection();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello =
        path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // GET /invitation =>
    let invite = path!("invite").map(|| "Hello!");

    // GET /auth =>
    let auth = path!("auth").map(|| "Hello!");

    // GET /register =>
    let register =
        path!("register" / String).map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(hello.or(invite).or(auth).or(register));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

mod filters {
    use warp::Filter;

    /// The 3 filters combined.
    pub fn auth_filter(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        get_user(db.clone()).or(login(db.clone())).or(logout(db))
    }

    /// GET /auth
    pub fn get_user(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("todos")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_db(db))
            .and_then(handlers::list_todos)
    }

    /// POST /auth with JSON body
    pub fn login(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("auth")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::create_todo)
    }

    /// DELETE /todos/:id
    pub fn logout(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        // We'll make one of our endpoints admin-only to show how authentication filters are used
        let admin_only = warp::header::exact("authorization", "Bearer admin");

        warp::path!("auth" / u64)
            // It is important to put the auth check _after_ the path filters.
            // If we put the auth check before, the request `PUT /todos/invalid-string`
            // would try this filter and reject because the authorization header doesn't match,
            // rather because the param is wrong for that other path.
            .and(admin_only)
            .and(warp::delete())
            .and(with_db(db))
            .and_then(handlers::delete_todo)
    }

    fn with_db(
        db: Db,
    ) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone
    {
        warp::any().map(move || db.clone())
    }

    fn json_body(
    ) -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
