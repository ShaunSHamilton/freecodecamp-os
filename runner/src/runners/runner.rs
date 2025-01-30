use crate::Tests;

pub trait Runner {
    fn run(&self, tests: &Tests) -> Result<String, RunnerError>;
    fn run_str(&self, code: &str) -> Result<String, RunnerError>;
}

pub enum RunnerError {
    Io(std::io::Error),
}
