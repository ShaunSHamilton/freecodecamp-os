use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path,
    },
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

mod ws;

const INDEX_HTML: &str = include_str!("../../client/dist/index.html");
const INDEX_JS: &str = include_str!("../../client/dist/assets/index.js");
const INDEX_CSS: &str = include_str!("../../client/dist/assets/style.css");
const LATO_FONT: &[u8] = include_bytes!("../../client/dist/assets/Lato-Regular.woff");

async fn handle_ws_connection(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn handle_index() -> Html<&'static str> {
    Html(&INDEX_HTML)
}

async fn handle_assets(Path(file_name): Path<String>) -> impl IntoResponse {
    let file_name = file_name.as_str();
    let content_type = match file_name {
        "index.js" => "application/javascript",
        "style.css" => "text/css",
        "Lato-Regular.woff" => "font/woff",
        _ => "text/plain",
    };

    let content = match file_name {
        "index.js" => INDEX_JS.as_bytes(),
        "style.css" => INDEX_CSS.as_bytes(),
        "Lato-Regular.woff" => LATO_FONT,
        _ => b"",
    };

    (
        [
            (CONTENT_TYPE, content_type),
            (CACHE_CONTROL, "public, max-age=31536000"),
        ],
        content,
    )
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handle_index))
        .route("/assets/:file_name", get(handle_assets))
        .route("/ws", get(handle_ws_connection));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
