mod routes;

use axum::Router;
use routes::create_routes;

pub async fn run() {
    let app: Router = create_routes();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}