use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub project_path: PathBuf,
    pub hooks_path: PathBuf,
    pub test_paths: Vec<PathBuf>,
}
