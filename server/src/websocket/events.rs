use serde::{Deserialize, Serialize};

use crate::FreeCodeCampConf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: uuid::Uuid,
    pub kind: EventKind,
}

// Example events:
// - Update: updates all data
// - Run Tests: Runs tests for a project>lesson

#[derive(Debug, Serialize, Deserialize)]
pub enum EventKind {
    Update(Update),
    RunTests(RunTests),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub projects: Vec<Project>,
    pub free_code_camp_config: FreeCodeCampConf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: u16,
    pub title: String,
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: u16,
    pub title: String,
    pub description: String,
    pub tests: Vec<Test>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    pub test_string: String,
    pub test_code: String,
    pub runner: Runner,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Runner {
    Node,
    Rust,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunTests {
    pub project_id: u16,
    pub lesson_id: u16,
}
