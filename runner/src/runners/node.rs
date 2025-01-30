use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command, str::FromStr};

use crate::Tests;

use super::runner::{Runner, RunnerError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(default = "default_path")]
    dir_path: PathBuf,
}

fn default_path() -> PathBuf {
    PathBuf::from("./.fcc-tests/node")
}

impl Runner for Node {
    fn run(&self, tests: &Tests) -> Result<String, RunnerError> {
        let index_path = PathBuf::from_str("./.fcc-tests/node/src/index.js")
            .expect("Explicit &str path is valid");

        let mut test_functions = Vec::new();
        let mut function_names = Vec::new();
        for i in 0..tests.len() {
            let test = &tests[i];
            let hooks = &tests.hooks;
            let function_name = format!("test_{}", i);
            let code = format!(
                r"
async function {function_name}() {{
    {b}
    {t}
    {a}
}}
            ",
                t = test.test_code,
                b = hooks
                    .iter()
                    .find_map(|hook| match hook {
                        config::Hook::BeforeEach(code) => Some(code),
                        _ => None,
                    })
                    .unwrap_or(&"".to_owned()),
                a = hooks
                    .iter()
                    .find_map(|hook| match hook {
                        config::Hook::AfterEach(code) => Some(code),
                        _ => None,
                    })
                    .unwrap_or(&"".to_owned()),
            );
            test_functions.push(code);
            function_names.push(function_name);
        }

        let test_functions = test_functions.join("\n");
        let function_calls = function_names
            .iter()
            .map(|f| format!("{f}();"))
            .collect::<Vec<String>>()
            .join("\n");
        let before_all = tests
            .hooks
            .iter()
            .find_map(|hook| match hook {
                config::Hook::BeforeAll(code) => Some(code.to_owned()),
                _ => None,
            })
            .unwrap_or_default();
        let after_all = tests
            .hooks
            .iter()
            .find_map(|hook| match hook {
                config::Hook::AfterAll(code) => Some(code.to_owned()),
                _ => None,
            })
            .unwrap_or_default();

        let code = format!(
            r"
import * as __helpers from './helpers.js';

async function main() {{
    {before_all}
    {function_calls}
    {after_all}
}}

await main();

{test_functions}
"
        );

        // Write the code to file
        let code_path = self.dir_path.join("src/index.js");

        std::fs::write(&code_path, code).expect("Failed to write code to file");

        // Run the code
        let output = Command::new("node")
            .arg(&index_path)
            .current_dir(&self.dir_path)
            .output()
            .expect("Failed to execute command");

        Ok(String::from_utf8(output.stdout).unwrap())
    }

    fn run_str(&self, code: &str) -> Result<String, RunnerError> {
        Ok("".to_string())
    }
}
