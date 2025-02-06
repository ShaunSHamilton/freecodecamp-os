use axum::{
    routing::{get, post},
    Router,
};
use state::AppState;
use tower_http::services::ServeDir;
use utils::read_config;

mod errors;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    let config = match read_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("ERROR:");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let app_state = AppState {
        config: config.clone(),
    };

    let mut app = Router::new()
        .route("/", get(routes::handle_index))
        .route("/assets/{path}", get(routes::handle_assets))
        .route(
            "/projects/{project_id}/lessons/{lesson_id}",
            get(routes::handle_project_lesson),
        )
        .route("/projects/{project_id}", get(routes::handle_get_project))
        .route("/projects", get(routes::handle_get_projects))
        .route("/config", get(routes::handle_get_config))
        .route("/state", get(routes::handle_get_state))
        .route("/state", post(routes::handle_post_state))
        .route(
            "/projects/{project_id}/lessons/{lesson_id}/reset",
            post(routes::handle_lesson_reset),
        )
        .route("/projects/{project_id}", post(routes::handle_project_reset))
        .route(
            "/project/{project_id}/{lesson_id}",
            post(routes::handle_post_project),
        )
        .route("/tests/run", post(routes::handle_run_tests))
        .route("/tests/cancel", post(routes::handle_cancel_tests))
        .with_state(app_state);

    for stat in config.client._static.iter() {
        app = app.nest_service(stat.0.to_str().unwrap(), ServeDir::new(stat.1));
    }

    let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
    println!(
        "Listening on http://127.0.0.1:{}",
        listener.local_addr().unwrap().port()
    );

    axum::serve(listener, app).await.unwrap();
}
