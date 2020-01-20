use warp::{self, path, Filter};

/**
 * TO-DO: router, method filter
*/
#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // GET /invitation =>
    let invite = path!("invite").map(|| "Hello!");

    // GET /auth =>
    let auth = path!("auth").map(|| "Hello!");

    // GET /register =>
    let register = path!("register" / String).map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(hello.or(invite).or(auth).or(register));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
