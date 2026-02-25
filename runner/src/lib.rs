///! This library orchestrates the runners for a given lesson.
///
/// Actions are run in order by the various runners.
/// 1) Lib divies all actions into their respective runners
/// 2) Runners wait for lib to signal when to advance
/// 3) Lib signals runner to advance
/// 4) Runner signals when it has made progress (finished an action)
/// 5) Lib signals relevant runner to advance next action
pub mod error;
mod manifest;
mod runner;
pub use runner::Runner;
pub mod runners;
