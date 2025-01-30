use std::collections::HashMap;

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
    pub id: u16,
    pub dashed_name: String,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub description: String,
    // pub hooks: Vec<Hook>,
    pub id: u16,
    // pub meta: HashMap<String, String>,
    // pub seed: Vec<Seed>,
    // pub tests: Vec<Test>,
    // pub title: String,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    // pub runner: Runner,
    pub test_string: String,
    pub test_code: String,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub enum Hook {
    BeforeAll(String),
    BeforeEach(String),
    AfterAll(String),
    AfterEach(String),
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seed {
    // pub runner: Runner,
    pub seed_code: String,
}
