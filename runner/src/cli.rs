use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    /// The code runner to use
    #[arg(short, long)]
    pub runner: Runner,
    /// The code to run
    pub content: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Runner {
    Node,
    Rust,
    Python,
    Bash,
}
