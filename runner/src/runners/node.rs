use config::Project;
use std::{
    io::{Read, Write},
    process::Command,
};
use tempfile::NamedTempFile;
use tracing::error;

use crate::{
    error::Error,
    manifest::Manifest,
    runner::{Hooks, Test},
    Runner,
};

static NODE_ENTRY: &str = include_str!("../../scripts/node/index.js");
static NODE_WORKER: &str = include_str!("../../scripts/node/test-worker.js");

pub struct Node;

impl Runner for Node {
    fn execute(project: Project, tests: Vec<Test>, hooks: Hooks) -> Result<Vec<Test>, Error> {
        // Ensure test dir exists
        let exists = std::fs::exists("../.test")?;
        if !exists {
            std::fs::create_dir_all("../.test")?;
        }

        let mut entry = NamedTempFile::new_in("../.test").unwrap();
        entry.write_all(NODE_ENTRY.as_bytes()).unwrap();
        let mut test_worker = NamedTempFile::new_in("../.test").unwrap();
        test_worker.write_all(NODE_WORKER.as_bytes()).unwrap();

        let mut project_file = NamedTempFile::new_in("../.test").unwrap();
        project_file
            .write_all(serde_json::to_string(&project).unwrap().as_bytes())
            .unwrap();
        let mut hooks_file = NamedTempFile::new_in("../.test").unwrap();

        hooks_file
            .write_all(serde_json::to_string(&hooks).unwrap().as_bytes())
            .unwrap();

        // one test file per test
        let mut test_files = vec![];

        for test in tests {
            let mut test_file = NamedTempFile::new_in("../.test").unwrap();
            test_file
                .write_all(serde_json::to_string(&test).unwrap().as_bytes())
                .unwrap();
            test_files.push(test_file);
        }

        let mut manifest_file = NamedTempFile::new_in("../.test").unwrap();

        let manifest = Manifest {
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
}

#[cfg(test)]
mod tests {

    use config::{Project, TestState};

    use crate::{
        runner::{Hooks, Test},
        runners::node::Node,
        Runner,
    };

    #[test]
    fn node_execute() {
        let tests = vec![
            Test {
                code: "assert.equal(b, 1);".to_string(),
                id: 0,
                state: TestState::Neutral,
            },
            Test {
                code: "assert.equal(b, 2);".to_string(),
                id: 1,
                state: TestState::Neutral,
            },
        ];

        let project = Project {
            title: Default::default(),
            description: Default::default(),
            id: 0,
            is_public: true,
            lessons: vec![],
        };

        let hooks = Hooks {
            before_all: Default::default(),
            before_each: "const b = 1;".to_string(),
            after_all: Default::default(),
            after_each: Default::default(),
        };

        let tests = Node::execute(project, tests, hooks).unwrap();

        let passed = tests.get(0).unwrap();
        let failed = tests.get(1).unwrap();

        assert_eq!(passed.state, TestState::Passed);

        match &failed.state {
            TestState::Failed(_) => {}
            a => panic!("expected {a:?} to be {:?}", TestState::Failed("any".into())),
        }
    }
}
