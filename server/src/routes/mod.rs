#![allow(unused_variables)]
use std::{path::PathBuf, str::FromStr};

use axum::{
    extract::Path,
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
    response::{Html, IntoResponse, Response},
    Json,
};

use include_dir::{include_dir, Dir};

use config::{FreeCodeCampConf, Lesson, LessonMarker, Project, State};

use crate::utils::{read_config, read_lesson, read_projects, read_state, set_state};

const INDEX_HTML: &str = include_str!("../../../client/dist/index.html");

static DIST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../client/dist");

pub async fn handle_assets(Path(path): Path<PathBuf>) -> Response {
    // let manifest = serde_json::from_str::<HashMap<String, Manifest>>(MANIFEST).unwrap();
    println!("{path:?}");

    let content_type = match path.extension() {
        Some(ext) => match ext.to_str().unwrap() {
            "js" => "application/javascript",
            "css" => "text/css",
            _ => "application/octet-stream",
        },
        None => "application/octet-stream",
    };

    let parent = PathBuf::from_str("assets").unwrap();

    let body = DIST_DIR.get_file(parent.join(path)).unwrap().contents();

    let body = IntoResponse::into_response(body);
    Response::builder()
        .header(CACHE_CONTROL, "public, max-age=31536000, immutable")
        .header(CONTENT_TYPE, content_type)
        .body(body.into_body())
        .unwrap()
}

pub async fn handle_index() -> Html<&'static str> {
    Html(&INDEX_HTML)
}

pub async fn handle_project_lesson(
    Path((project_id, lesson_id)): Path<(usize, usize)>,
) -> Json<Lesson> {
    let lesson = read_lesson(project_id, lesson_id);

    Json(lesson)
}

pub async fn handle_get_config() -> Json<FreeCodeCampConf> {
    let config = read_config();
    Json(config)
}

pub async fn handle_get_state() -> Json<State> {
    let state = read_state();

    Json(state)
}

pub async fn handle_post_state(Json(state): Json<State>) {
    set_state(state);
}

pub async fn handle_lesson_reset(Path((project_id, lesson_id)): Path<(usize, usize)>) {
    todo!()
}

pub async fn handle_project_reset(Path(project_id): Path<usize>) {
    todo!()
}

pub async fn handle_get_projects() -> Json<Vec<Project>> {
    let projects = read_projects();
    Json(projects)
}

pub async fn handle_get_project(Path(project_id): Path<usize>) -> Json<Project> {
    let projects = read_projects();
    let project = projects
        .into_iter()
        .find(|p| p.meta.id == project_id)
        .unwrap();
    Json(project)
}

/// Handles the running of tests.
pub async fn handle_run_tests(Json(_meta): Json<LessonMarker>) {
    todo!();
}

/// Handles a lesson submission.
pub async fn handle_post_project(Path((project_id, lesson_id)): Path<(usize, usize)>) {
    todo!();
}

pub async fn handle_cancel_tests() {
    todo!();
}
