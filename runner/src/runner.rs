use config::{Project, TestState};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub id: usize,
    pub code: String,
    pub state: TestState,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Hooks {
    pub before_all: String,
    pub before_each: String,
    pub after_all: String,
    pub after_each: String,
}

pub trait Runner {
    fn execute(project: Project, tests: Vec<Test>, hooks: Hooks) -> Result<Vec<Test>, Error>;
}
