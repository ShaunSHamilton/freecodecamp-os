use clap::Parser;
use std::{
    io::Write,
    process::{Command, Stdio},
};
use tempfile::NamedTempFile;

mod cli;
use cli::Cli;

static NODE_SCRIPT: &str = include_str!("../scripts/node/index.js");

fn main() {
    let args = Cli::parse();

    match args.runner {
        cli::Runner::Node => {
            let test_meta_json = args.content;

            // Create a temporary file for the Node.js script
            let mut script_file = NamedTempFile::new().expect("Failed to create temp file");
            script_file
                .write_all(NODE_SCRIPT.as_bytes())
                .expect("Failed to write script to temp file");

            let status = Command::new("node")
                .arg(script_file.path()) // Pass the script file as an argument
                .stdin(Stdio::piped())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .and_then(|mut child| {
                    if let Some(mut stdin) = child.stdin.take() {
                        stdin
                            .write_all(test_meta_json.as_bytes())
                            .expect("Failed to write JSON to stdin");
                    }
                    child.wait()
                })
                .expect("Failed to run Node.js script");

            if !status.success() {
                eprintln!("Node.js script execution failed");
                std::process::exit(1);
            }
        }
        cli::Runner::Rust => {
            unimplemented!()
        }
        cli::Runner::Python => {
            unimplemented!()
        }
        cli::Runner::Bash => {
            unimplemented!()
        }
    };
}
