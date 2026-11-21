use axum::{
    Router,
    routing::get,
};
use tower_http::services::ServeDir;

mod db;
mod models;
mod handlers;
mod token_auth;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let conn = db::connect().unwrap();
    db::init::init_db(&conn).unwrap();
    let app = Router::new()
        .route("/sign_up", get(handlers::signup::sign_up_handler))
        .route("/log_in", get(handlers::login::login_handler))
        .nest_service("/static", ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}