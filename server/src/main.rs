use axum::{
    routing::{get, post},
    Router,
};

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
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
        .route("/tests/cancel", post(routes::handle_cancel_tests));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: http://localhost:3000/");
    axum::serve(listener, app).await.unwrap();
}
