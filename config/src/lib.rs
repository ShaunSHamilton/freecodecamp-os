use std::{collections::HashMap, path::PathBuf, str::FromStr};

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
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub description: String,
    pub meta: ProjectMeta,
    pub title: String,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub id: u16,
    pub is_public: bool,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub description: String,
    pub code_meta: CodeMeta,
    pub id: u16,
    // pub meta: HashMap<String, String>,
    // pub title: String,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Test {
    pub runner: Runner,
    pub text: String,
    pub code: String,
    pub state: TestState,
    pub id: u16,
}

#[typeshare]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Runner {
    Node,
    Rust,
    Python,
    Bash,
}

impl FromStr for Runner {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let run = s.split("runner=").collect::<Vec<&str>>()[1];

        match run {
            "node" => Ok(Runner::Node),
            "rust" => Ok(Runner::Rust),
            "python" => Ok(Runner::Python),
            "bash" => Ok(Runner::Bash),
            _ => Err(()),
        }
    }
}

impl From<String> for Runner {
    fn from(s: String) -> Self {
        let s = s.split("runner=").collect::<Vec<&str>>();
        let run = s.get(1).unwrap_or(&"node");
        match *run {
            "node" => Runner::Node,
            "rust" => Runner::Rust,
            "python" => Runner::Python,
            "bash" => Runner::Bash,
            _ => panic!("Invalid runner"),
        }
    }
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

// #[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(tag = "_type", content = "content")]]
pub enum Hook {
    BeforeAll(Test),
    BeforeEach(Test),
    AfterAll(Test),
    AfterEach(Test),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeMeta {
    pub hooks: Vec<Hook>,
    pub tests: Vec<Test>,
    pub seed: Vec<Seed>,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Seed {
    pub seed_code: String,
    pub id: u16,
    pub kind: SeedKind,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SeedKind {
    Code(PathBuf),
    Command,
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
