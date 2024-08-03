mod routes;

use sea_orm::Database;

pub async fn run(database_url: &str) {
    let database = Database::connect(database_url).await.unwrap();
    let app = routes::create_routes(database);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
