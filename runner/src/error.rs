#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Froms
    #[error("{0}")]
    FS(#[from] std::io::Error),
}
