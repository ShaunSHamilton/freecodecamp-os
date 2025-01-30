pub mod runners;

pub struct Test {
    pub runner: Box<dyn runners::runner::Runner>,
    pub test_string: String,
    pub test_code: String,
}

pub type Tests = Vec<Test>;
