use axum::{
    extract::ws::{Message, Utf8Bytes},
    routing::{get, post},
    Router,
};
use notify::{Event, RecursiveMode, Watcher};
use state::AppState;
use tokio::sync::{watch, Mutex};
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

    let (tx, rx) = watch::channel(Message::Text(Utf8Bytes::from_static("")));

    let app_state = AppState {
        config: config.clone(),
        rx,
    };

    tokio::spawn(async move {
        let (notify_tx, notify_rx) = std::sync::mpsc::channel::<notify::Result<Event>>();

        // Use recommended_watcher() to automatically select the best implementation
        // for platform. The `EventHandler` passed to this constructor can be a
        // closure, a `std::sync::mpsc::Sender`, a `crossbeam_channel::Sender`, or
        // another type the trait is implemented for.
        let mut watcher = notify::recommended_watcher(notify_tx).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        let options = notify::Config::default()
            .with_poll_interval(std::time::Duration::from_secs(1))
            .with_follow_symlinks(false);
        watcher.configure(options).unwrap();
        watcher
            .watch(std::path::Path::new("."), RecursiveMode::Recursive)
            .unwrap();
        // Block forever, printing out events as they come in
        for res in notify_rx {
            match res {
                Ok(event) => {
                    println!("event: {:?}", event);
                    // TODO: Run lesson
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    let mut app = Router::new()
        .route("/", get(routes::handle_index))
        .route("/ws", get(routes::handle_websocket))
        .fallback_service(ServeDir::new("client/dist"))
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
