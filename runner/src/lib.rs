///! This library orchestrates the runners for a given lesson.
///
/// Actions are run in order by the various runners.
/// 1) Lib divies all actions into their respective runners
/// 2) Runners wait for lib to signal when to advance
/// 3) Lib signals runner to advance
/// 4) Runner signals when it has made progress (finished an action)
/// 5) Lib signals relevant runner to advance next action
use config::{Lesson, Runner, Seed};
use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use tempfile::NamedTempFile;

static NODE_SCRIPT: &str = include_str!("../scripts/node/dist/index.js");

pub fn run_lesson(lesson: Lesson) {
    // Collect all actions into list of order to run
    // Once a runner is encountered, create child process to continue using.

    let mut file = NamedTempFile::new().unwrap();
    file.write_all(NODE_SCRIPT.as_bytes()).unwrap();
    let mut child = Command::new("node")
        .arg("-i")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child.stdout.take().unwrap();
    let mut stdout = BufReader::new(stdout);
    let stderr = child.stderr.take().unwrap();
    let mut stderr = BufReader::new(stderr);

    let mut line = String::new();

    thread::spawn(move || loop {
        line.clear();
        match stdout.read_to_string(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                println!("stdout: {}", line);
            }
            Err(e) => {
                println!("Error reading stdout: {}", e);
                break;
            }
        }
    });

    let mut line = String::new();
    thread::spawn(move || loop {
        line.clear();
        match stderr.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                println!("stderr: {}", line);
            }
            Err(e) => {
                println!("Error reading stderr: {}", e);
                break;
            }
        }
    });

    let mut stdin = child.stdin.take().unwrap(); // Take ownership
                                                 // thread::sleep(Duration::from_secs(1));
    send_to_stdin(
        &mut stdin,
        &format!(".load {} \n", file.path().to_str().unwrap()),
    );
    for a in lesson.before_all {
        match a {
            Seed::Command {
                runner: Runner::Node,
                code,
            } => {
                thread::sleep(Duration::from_secs(1));
                send_to_stdin(&mut stdin, &code);
            }
            _ => {
                unimplemented!()
            }
        }
    }

    let before_each = lesson.before_each;
    let after_each = lesson.after_each;

    for test in lesson.tests {
        for a in before_each.iter() {
            match a {
                Seed::Command {
                    runner: Runner::Node,
                    code,
                } => {
                    // thread::sleep(Duration::from_secs(1));
                    send_to_stdin(&mut stdin, code);
                }
                _ => {
                    unimplemented!()
                }
            }
        }
        match test.runner {
            Runner::Node => {
                // thread::sleep(Duration::from_secs(1));
                send_to_stdin(&mut stdin, &test.code);
            }
            _ => unimplemented!(),
        }

        for a in after_each.iter() {
            match a {
                Seed::Command {
                    runner: Runner::Node,
                    code,
                } => {
                    // thread::sleep(Duration::from_secs(1));
                    send_to_stdin(&mut stdin, code);
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }

    for a in lesson.after_all {
        match a {
            Seed::Command {
                runner: Runner::Node,
                code,
            } => {
                // thread::sleep(Duration::from_secs(1));
                send_to_stdin(&mut stdin, &code);
            }
            _ => {
                unimplemented!()
            }
        }
    }

    // thread::sleep(Duration::from_secs(1));
    send_to_stdin(&mut stdin, ".exit");
    stdin.flush().unwrap(); // Important: Flush stdin to ensure commands are sent
    let status = child.wait().unwrap();

    println!("Node process exited with status: {}", status);
}

fn send_to_stdin(stdin: &mut std::process::ChildStdin, code: &str) {
    stdin.write_all(code.as_bytes()).unwrap();
    stdin.write_all("\n".as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Lesson, Runner, Seed, Test};

    #[test]
    fn test_run_lesson() {
        let lesson = Lesson {
            after_all: vec![],
            after_each: vec![],
            before_all: vec![Seed::Command {
                runner: Runner::Node,
                code: "const b = 1;".to_string(),
            }],
            before_each: vec![],
            description: String::new(),
            hints: vec![],
            id: 0,
            seeds: vec![],
            tests: vec![Test {
                code: "assert.equal(b, 2);".to_string(),
                id: 0,
                runner: Runner::Node,
                text: "Some test.".to_string(),
            }],
        };

        run_lesson(lesson);
    }
}
