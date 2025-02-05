use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct FreeCodeCampConf {
    pub client: Client,
    pub version: String,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    landing: HashMap<Locale, Landing>,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct Landing {
    pub title: String,
    pub description: String,
    pub faq_link: String,
    pub faq_text: String,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Locale {
    En,
    Af,
}

#[typeshare]
#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub meta: ProjectMeta,
    pub lessons: Vec<Lesson>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub title: String,
    pub description: String,
    pub id: usize,
    pub is_public: bool,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub meta: LessonMeta,
    pub tests: Vec<Test>,
    pub seeds: Vec<Seed>,
    pub hooks: Hook,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonMeta {
    pub description: String,
    pub id: usize,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    pub runner: String,
    pub text: String,
    pub code: String,
    pub id: usize,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Seed {
    Command { runner: String, code: String },
    File { path: PathBuf, content: String },
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    pub before_all: Vec<Seed>,
    pub before_each: Vec<Seed>,
    pub after_all: Vec<Seed>,
    pub after_each: Vec<Seed>,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TestState {
    /// Test has not run yet, or was cancelled
    Neutral,
    /// Test passed
    Passed,
    /// Test failed with output
    Failed(String),
    /// Test is running
    Running,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub locale: Locale,
    pub completed_lessons: Vec<CompletedLesson>,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedLesson {
    pub project_id: u16,
    pub lesson_id: u16,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LessonMarker {
    pub project_id: u16,
    pub lesson_id: u16,
}
