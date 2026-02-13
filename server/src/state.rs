use axum::extract::ws::Message;
use config::FreeCodeCampConf;

#[derive(Clone)]
pub struct AppState {
    pub config: FreeCodeCampConf,
    pub rx: tokio::sync::watch::Receiver<Message>,
}
