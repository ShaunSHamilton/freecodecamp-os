#![allow(unused_variables)]
use std::{path::PathBuf, process::Command, str::FromStr};

use axum::{
    extract::{Path, State},
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
    response::{Html, IntoResponse, Response},
    Json,
};

use include_dir::{include_dir, Dir};

use config::{FreeCodeCampConf, Lesson, LessonMarker, Project};

use crate::{
    errors::AppError,
    state::AppState,
    utils::{read_config, read_lesson, read_projects, read_state, set_state},
};

const INDEX_HTML: &str = include_str!("../../../client/dist/index.html");

static DIST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../client/dist");

pub async fn handle_assets(Path(path): Path<PathBuf>) -> Response {
    println!("{path:?}");

    let content_type = match path.extension() {
        Some(ext) => match ext.to_str().unwrap() {
            "js" => "application/javascript",
            "css" => "text/css",
            "html" => "text/html",
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

pub async fn handle_get_config() -> Result<Json<FreeCodeCampConf>, AppError> {
    let config = read_config()?;
    Ok(Json(config))
}

pub async fn handle_get_state(State(state): State<AppState>) -> Json<config::State> {
    let state = read_state(&state.config);

    Json(state)
}

pub async fn handle_post_state(Json(state): Json<config::State>) {
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
    let project = projects.into_iter().find(|p| p.id == project_id).unwrap();
    Json(project)
}

/// Handles the running of tests.
pub async fn handle_run_tests(State(state): State<AppState>, Json(meta): Json<LessonMarker>) {
    // Get lesson
    let lesson = read_lesson(meta.project_id, meta.lesson_id);
    // Call runner
    let runners = state.config.runners;
    // Run before alls
    let before_all = lesson.before_all;

    for test in lesson.tests {
        // Run before each
        // Run test
        let mut runner = runners.get(&test.runner).unwrap().split_whitespace();
        let runner_name = runner.next().unwrap();
        let runner_args = runner.collect::<Vec<&str>>();
        let output = Command::new(runner_name)
            .args(runner_args)
            .arg(test.code)
            .output()
            .unwrap();

        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        // Run after each
    }

    // Run after alls
    let after_all = lesson.after_all;
}

/// Handles a lesson submission.
pub async fn handle_post_project(Path((project_id, lesson_id)): Path<(usize, usize)>) {
    todo!();
}

pub async fn handle_cancel_tests() {
    todo!();
}
