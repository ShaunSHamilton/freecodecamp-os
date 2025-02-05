use config::Project;
use convert::FromMdast;
use markdown::{mdast::Node, to_mdast, ParseOptions};

mod convert;
mod node_ext;

#[derive(Debug)]
pub enum ParseError {
    BadNode(String),
}

impl From<Option<Node>> for ParseError {
    fn from(node: Option<Node>) -> Self {
        ParseError::BadNode(format!("{:?}", node))
    }
}

trait Context<T> {
    fn context(self, msg: &str) -> Result<T, ParseError>;
}

impl<T> Context<T> for Option<T> {
    fn context(self, msg: &str) -> Result<T, ParseError> {
        self.ok_or_else(|| ParseError::BadNode(msg.to_string()))
    }
}

pub fn parse_project(markdown_str: &str) -> Result<Project, ParseError> {
    let options = ParseOptions::gfm();
    let node = to_mdast(markdown_str, &options).unwrap();
    let node = node.children().unwrap().to_owned();
    // Serialize the nodes to a Project
    let project = Project::from_mdast(node)?;
    Ok(project)
}
