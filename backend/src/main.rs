use axum::{routing::get, Router};
use tera::Tera;
use tower_http::services::ServeDir;

mod handlers;
mod utils;

use handlers::{
    get_about, get_cours_summary, get_exercices_summary, get_single_course, get_single_exercice,
};

#[derive(Clone)]
struct AppState {
    tera: Tera,
}

#[tokio::main]
async fn main() {
    let tera = match Tera::new("templates/**/*.html.tera") {
        Ok(t) => t,
        Err(e) => {
            panic!("Tera instanciation error : {}", e);
        }
    };

    let app_state = AppState { tera };

    let app = Router::new()
        .route("/", get(get_cours_summary))
        .route("/cours", get(get_cours_summary))
        .route("/cours/{file_name}", get(get_single_course))
        .route("/exercices", get(get_exercices_summary))
        .route("/exercices/{file_name}", get(get_single_exercice))
        .route("/about", get(get_about))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}