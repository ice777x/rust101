use axum::{routing::get, Router};
use news::{
    routes::{create_feed, get_feed, root},
    Database,
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    println!("Server started at port 3000");
    let db = Arc::new(Mutex::new(Database::new()));
    let app = Router::new()
        .route("/", get(root))
        .route("/news", get(get_feed))
        .route("/news/create", get(create_feed))
        .with_state(db.clone());
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
