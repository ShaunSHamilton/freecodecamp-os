use clap::{Parser, ValueEnum};
// use config::TestMeta;

#[derive(Parser)]
pub struct Cli {
    /// The code runner to use
    #[arg(short, long)]
    pub runner: Runner,
    /// The code to run
    pub test_meta: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Runner {
    Node,
    Rust,
    Python,
    Bash,
}

// use clap::{Parser, ValueEnum};
// use config::TestMeta;

// #[derive(Parser)]
// pub struct Cli {
//     /// The code runner to use
//     #[arg(short, long)]
//     pub runner: Runner,
//     /// The code to run
//     #[arg(value_parser = test_meta_serializer)]
//     pub test_meta: TestMeta,
// }

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// pub enum Runner {
//     Node,
//     Rust,
//     Python,
//     Bash,
// }

// fn test_meta_serializer(val: &str) -> Result<TestMeta, String> {
//     let test_meta: TestMeta = serde_json::from_str(val).map_err(|e| e.to_string())?;
//     Ok(test_meta)
// }
