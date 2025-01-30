use axum::{
    routing::{get, post},
    Router,
};

// use websocket::events::Event;

// mod websocket;
// mod ws;

// async fn handle_ws_connection(ws: WebSocketUpgrade) -> Response {
//     ws.on_upgrade(handle_socket)
// }

// async fn handle_socket(mut socket: WebSocket) {
//     while let Some(msg) = socket.recv().await {
//         let msg = if let Ok(msg) = msg {
//             match msg {
//                 Message::Text(text) => {
//                     let event: Event = match serde_json::from_str(&text) {
//                         Ok(event) => event,
//                         Err(_) => continue,
//                     };
//                     println!("{:?}", event);
//                 }
//                 _ => continue,
//             }
//         } else {
//             // client disconnected
//             return;
//         };

//         if socket.send(msg).await.is_err() {
//             // client disconnected
//             return;
//         }
//     }
// }

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(routes::handle_index))
        .route("/assets/{path}", get(routes::handle_assets))
        .route(
            "/{project_id}/{lesson_id}",
            get(routes::handle_project_lesson),
        )
        .route("/projects", get(routes::handle_get_projects))
        .route("/config", get(routes::handle_get_config))
        .route("/config", post(routes::handle_post_config))
        .route("/reset-lesson", post(routes::handle_lesson_reset))
        .route("/reset-project", post(routes::handle_project_reset));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: http://localhost:3000/");
    axum::serve(listener, app).await.unwrap();
}
