use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Serialize;
use std::fs;
use tera::{Context, Tera};

#[derive(Serialize)]
pub struct FileInfo {
    file_name: String,
    title: String,
}

pub fn read_files_from_dir(path: &str) -> Vec<FileInfo> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let file_name = entry.file_name().into_string().unwrap_or_default();
                    let title = file_name
                        .strip_suffix(".html")
                        .unwrap_or(&file_name)
                        .replace('_', " ")
                        .to_uppercase();
                    files.push(FileInfo { file_name, title });
                }
            }
        }
    }
    files
}

pub fn render_template(tera: &Tera, template_name: &str, context: Context) -> Response {
    match tera.render(template_name, &context) {
        Ok(html) => Html(html).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template rendering error: {}", e),
        )
            .into_response(),
    }
}
