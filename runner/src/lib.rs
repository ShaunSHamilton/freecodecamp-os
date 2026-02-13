///! This library orchestrates the runners for a given lesson.
///
/// Actions are run in order by the various runners.
/// 1) Lib divies all actions into their respective runners
/// 2) Runners wait for lib to signal when to advance
/// 3) Lib signals runner to advance
/// 4) Runner signals when it has made progress (finished an action)
/// 5) Lib signals relevant runner to advance next action
use config::{FreeCodeCampConf, Lesson, Project, Seed, TestState};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::PathBuf,
    process::Command,
};
use tempfile::NamedTempFile;
use tracing::{debug, error};

use crate::manifest::Manifest;

mod manifest;

static NODE_ENTRY: &str = include_str!("../scripts/node/index.js");
static NODE_WORKER: &str = include_str!("../scripts/node/test-worker.js");

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub id: usize,
    pub code: String,
    pub state: TestState,
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Hooks {
    before_all: String,
    before_each: String,
    after_all: String,
    after_each: String,
}

pub fn run(config: FreeCodeCampConf, lesson: Lesson) -> Result<Vec<Test>, String> {
    // Collect all actions into list of order to run
    // Once a runner is encountered, create child process to continue using.

    let mut entry = NamedTempFile::new_in("../.test").unwrap();
    entry.write_all(NODE_ENTRY.as_bytes()).unwrap();
    let mut test_worker = NamedTempFile::new_in("../.test").unwrap();
    test_worker.write_all(NODE_WORKER.as_bytes()).unwrap();

    let mut config_file = NamedTempFile::new_in("../.test").unwrap();
    config_file
        .write_all(serde_json::to_string(&config).unwrap().as_bytes())
        .unwrap();
    let mut project_file = NamedTempFile::new_in("../.test").unwrap();
    let project = Project {
        title: "Test Project 0".to_string(),
        description: "Test project for unit tests".to_string(),
        id: 0,
        is_public: true,
        lessons: vec![lesson.clone()],
    };
    project_file
        .write_all(serde_json::to_string(&vec![project]).unwrap().as_bytes())
        .unwrap();
    let mut hooks_file = NamedTempFile::new_in("../.test").unwrap();
    let hooks = Hooks {
        before_all: lesson.before_all.map_or(String::new(), |s| match s {
            Seed::Command { runner, code } => code,
            _ => String::new(),
        }),
        before_each: lesson.before_each.map_or(String::new(), |s| match s {
            Seed::Command { runner, code } => code,
            _ => String::new(),
        }),
        after_all: lesson.after_all.map_or(String::new(), |s| match s {
            Seed::Command { runner, code } => code,
            _ => String::new(),
        }),
        after_each: lesson.after_each.map_or(String::new(), |s| match s {
            Seed::Command { runner, code } => code,
            _ => String::new(),
        }),
    };
    hooks_file
        .write_all(serde_json::to_string(&hooks).unwrap().as_bytes())
        .unwrap();

    // one test file per test
    let mut test_files = vec![];

    for test in lesson.tests {
        let mut test_file = NamedTempFile::new_in("../.test").unwrap();
        let path = test_file.path().to_path_buf();
        let test = Test {
            id: test.id,
            code: test.code,
            state: TestState::Neutral,
            path: path.clone(),
        };
        test_file
            .write_all(serde_json::to_string(&test).unwrap().as_bytes())
            .unwrap();
        test_files.push(test_file);
    }

    let mut manifest_file = NamedTempFile::new_in("../.test").unwrap();
    let manifest = Manifest {
        config_path: config_file.path().to_path_buf(),
        project_path: project_file.path().to_path_buf(),
        hooks_path: hooks_file.path().to_path_buf(),
        test_paths: test_files.iter().map(|t| t.path().to_path_buf()).collect(),
    };

    manifest_file
        .write_all(serde_json::to_string(&manifest).unwrap().as_bytes())
        .unwrap();

    let mut child = Command::new("node")
        .arg(entry.path())
        .env("MANIFEST_PATH", &manifest_file.path())
        .env("TEST_WORKER_PATH", &test_worker.path())
        .current_dir("../")
        .spawn()
        .unwrap();

    let status = child.wait().expect("failed to wait on child");

    if !status.success() {
        eprintln!("Node.js test runner exited with error: {:?}", status.code());
    }

    let tests = test_files
        .iter()
        .map(|f| {
            let mut buf = Vec::new();
            let mut file = f.reopen().unwrap();
            file.read_to_end(&mut buf).unwrap();
            match serde_json::from_slice(&buf) {
                Ok(v) => v,
                Err(e) => {
                    error!(error = ?e, "unable to parse test file");
                    panic!("{e:?}");
                }
            }
        })
        .collect();

    println!("{:#?}", tests);

    Ok(tests)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use config::{Client, Config, FreeCodeCampConf, Lesson, Runner, Seed, Test, TestState};

    use crate::run;

    #[test]
    fn test_run_lesson() {
        let lesson = Lesson {
            after_all: None,
            after_each: None,
            before_all: None,
            before_each: Some(Seed::Command {
                runner: Runner::Node,
                code: "const b = 1;".to_string(),
            }),
            description: String::new(),
            hints: vec![],
            id: 0,
            seeds: vec![],
            tests: vec![
                Test {
                    code: "assert.equal(b, 1);".to_string(),
                    id: 0,
                    runner: Runner::Node,
                    text: "Some passing test.".to_string(),
                },
                Test {
                    code: "assert.equal(b, 2);".to_string(),
                    id: 1,
                    runner: Runner::Node,
                    text: "Some failing test.".to_string(),
                },
            ],
        };

        let config = FreeCodeCampConf {
            client: Client {
                landing: Default::default(),
                _static: Default::default(),
            },
            version: "0.1.0".to_string(),
            addr: std::net::SocketAddr::from_str("0.0.0.0:8080").unwrap(),
            config: Config {
                state: PathBuf::from_str("../package.json").unwrap(),
            },
        };

        let tests = run(config, lesson).unwrap();

        let passed = tests.get(0).unwrap();
        let failed = tests.get(1).unwrap();

        assert_eq!(passed.state, TestState::Passed);

        match &failed.state {
            TestState::Failed(_) => {}
            a => panic!("expected {a:?} to be {:?}", TestState::Failed("any".into())),
        }
    }
}
