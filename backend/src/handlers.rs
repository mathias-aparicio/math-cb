use crate::utils::{read_files_from_dir, render_template};
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::fs;
use tera::Context;

pub async fn get_summary(
    state: AppState,
    content_dir: &str,
    context_key: &str,
    template_name: &str,
) -> impl IntoResponse {
    let files = read_files_from_dir(&format!("../frontend/html/{}", content_dir));
    let mut summary_context = Context::new();
    summary_context.insert(context_key, &files);

    let summary_html = match state.tera.render(template_name, &summary_context) {
        Ok(html) => html,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template rendering error: {}", e),
            )
                .into_response();
        }
    };

    let mut base_context = Context::new();
    base_context.insert("content", &summary_html);
    render_template(&state.tera, "base.html.tera", base_context)
}

pub fn render_static_page(state: AppState, file_path: String) -> impl IntoResponse {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let mut context = Context::new();
            context.insert("content", &content);
            render_template(&state.tera, "base.html.tera", context)
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

pub async fn get_single_content(
    state: AppState,
    file_name: String,
    content_dir: &str,
) -> impl IntoResponse {
    let path = format!("../frontend/html/{}/{}", content_dir, file_name);
    render_static_page(state, path)
}

pub async fn get_about(State(state): State<AppState>) -> impl IntoResponse {
    render_static_page(state, "../frontend/html/about.html".to_string())
}
pub async fn get_cours_summary(State(state): State<AppState>) -> impl IntoResponse {
    get_summary(
        state,
        "cours",
        "courses",
        "pages/cours_summary.html.tera",
    )
    .await
}

pub async fn get_exercices_summary(State(state): State<AppState>) -> impl IntoResponse {
    get_summary(
        state,
        "exercices",
        "exercices",
        "pages/exercices_summary.html.tera",
    )
    .await
}

pub async fn get_single_course(
    State(state): State<AppState>,
    Path(file_name): Path<String>,
) -> impl IntoResponse {
    get_single_content(state, file_name, "cours").await
}

pub async fn get_single_exercice(
    State(state): State<AppState>,
    Path(file_name): Path<String>,
) -> impl IntoResponse {
    get_single_content(state, file_name, "exercices").await
}
