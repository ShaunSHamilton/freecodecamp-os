use std::{collections::HashMap, net::SocketAddr, path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeCodeCampConf {
    pub client: Client,
    pub version: String,
    #[typeshare(serialize_as = "string")]
    pub addr: SocketAddr,
    pub config: Config,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub state: PathBuf,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub landing: HashMap<String, Landing>,
    #[serde(rename = "static")]
    pub _static: HashMap<PathBuf, PathBuf>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Landing {
    pub title: String,
    pub description: String,
    pub faq_link: String,
    pub faq_text: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    #[typeshare(serialized_as = "number")]
    pub id: usize,
    pub is_public: bool,
    pub lessons: Vec<Lesson>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub after_all: Option<Seed>,
    pub after_each: Option<Seed>,
    pub before_all: Option<Seed>,
    pub before_each: Option<Seed>,
    pub description: String,
    pub hints: Vec<Hint>,
    #[typeshare(serialized_as = "number")]
    pub id: usize,
    pub seeds: Vec<Seed>,
    pub tests: Vec<Test>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    #[typeshare(serialized_as = "number")]
    pub id: usize,
    pub text: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    pub code: String,
    #[typeshare(serialized_as = "number")]
    pub id: usize,
    pub runner: Runner,
    pub text: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Seed {
    Command {
        runner: Runner,
        code: String,
    },
    File {
        #[typeshare(serialized_as = "string")]
        path: PathBuf,
        content: String,
    },
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Runner {
    Node,
    Rust,
    Bash,
}

impl FromStr for Runner {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split("runner=").nth(1).unwrap();
        match s {
            "node" => Ok(Self::Node),
            "rust" => Ok(Self::Rust),
            "bash" => Ok(Self::Bash),
            _ => Err(()),
        }
    }
}

impl From<String> for Runner {
    fn from(s: String) -> Self {
        let s = s.split("runner=").nth(1).unwrap();
        match s {
            "node" => Self::Node,
            "rust" => Self::Rust,
            "bash" => Self::Bash,
            _ => panic!("Invalid runner"),
        }
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind", content = "content", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TestState {
    /// Test has not run yet, or was cancelled
    Neutral,
    /// Test passed
    Passed,
    /// Test failed with output
    Failed(serde_json::Value),
    /// Test is running
    Running,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub locale: String,
    pub completed_lessons: Vec<CompletedLesson>,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedLesson {
    pub project_id: usize,
    pub lesson_id: usize,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LessonMarker {
    pub project_id: usize,
    pub lesson_id: usize,
}
