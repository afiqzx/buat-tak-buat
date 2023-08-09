use std::sync::Arc;

use axum::extract::Extension;
use axum::{
    routing::{get, post},
    Router,
};
use buat_tak_buat_lib::path::*;
use buat_tak_buat_lib::*;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let todos = Extension(Arc::new(Mutex::new(TodoData::default())));

    let app = Router::new()
        .route("/", get(index))
        .route("/add_todo", post(add_todo))
        .layer(todos);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
