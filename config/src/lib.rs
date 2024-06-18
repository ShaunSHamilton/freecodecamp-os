use std::collections::HashMap;

pub struct FreeCodeCampConf {
    pub version: Version,
}

pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

pub struct Project {
    pub id: u16,
}

pub struct State {
    pub locale: Locale,
}

pub enum Locale {
    En,
    Af,
}

pub struct Lesson {
    pub description: String,
    pub hooks: Vec<Hook>,
    pub id: u16,
    pub meta: HashMap<String, String>,
    pub seed: Vec<Seed>,
    pub tests: Vec<Test>,
    pub title: String,
}

pub struct Test {
    pub runner: Runner,
    pub test_string: String,
    pub test_code: String,
}

pub enum Hook {
    BeforeAll(String),
    BeforeEach(String),
    AfterAll(String),
    AfterEach(String),
}

pub struct Seed {
    pub runner: Runner,
    pub seed_code: String,
}
