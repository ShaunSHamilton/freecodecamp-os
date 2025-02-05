use std::{path::PathBuf, str::FromStr};

use config::{Hint, Lesson, Project, Seed, Test};
use itertools::Itertools;
use markdown::mdast::{Heading, Node};
use serde_json::Value;

use crate::{
    node_ext::{NodeExt, Utils},
    Context, ParseError,
};

pub trait FromMdast {
    fn from_mdast(nodes: Vec<Node>) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl FromMdast for Project {
    fn from_mdast(nodes: Vec<Node>) -> Result<Self, ParseError> {
        let mut it = nodes.into_iter().peekable();

        let title = it
            .next()
            .as_heading()
            .context("# <TITLE>")?
            .children
            .stringify();

        let code = it.next().as_code().context("project meta missing")?;
        let json: Value = serde_json::from_str(&code.value).unwrap();

        let id: &Value = json.get("id").context("id missing")?;
        let id = id.as_u64().context("id incorrect type")?.clone() as usize;
        let is_public: &Value = json.get("is_public").context("is_public missing")?;
        let is_public = is_public
            .as_bool()
            .context("is_public incorrect type")?
            .clone();

        let description = it
            .by_ref()
            .peeking_take_while(|node| {
                if let Node::Heading(h) = node {
                    if h.depth == 2 {
                        if h.children.stringify().trim().parse::<usize>().is_ok() {
                            return false;
                        }
                    }
                }
                true
            })
            .collect::<Vec<Node>>()
            .stringify();

        let mut lessons = vec![];
        // Group nodes by lesson
        let c = it.fold(vec![], |mut acc, node| {
            match &node {
                Node::Heading(h) => {
                    if h.depth == 2 {
                        acc.push(vec![node]);
                    } else {
                        acc.last_mut().unwrap().push(node);
                    }
                }
                _ => {
                    acc.last_mut().unwrap().push(node);
                }
            }
            acc
        });

        for nodes in c {
            let lesson = Lesson::from_mdast(nodes)?;
            lessons.push(lesson);
        }

        let project = Project {
            lessons,
            title,
            description,
            id,
            is_public,
        };

        Ok(project)
    }
}

impl FromMdast for Lesson {
    fn from_mdast(nodes: Vec<Node>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut it = nodes.into_iter();
        let id = it
            .next()
            .as_heading()
            .context("lesson id")?
            .children
            .stringify()
            .trim()
            .parse::<usize>()
            .unwrap();

        let heads: Vec<Head> = it.fold(vec![], fold_into_heads);

        let mut hints = vec![];
        let mut after_all = vec![];
        let mut after_each = vec![];
        let mut before_all = vec![];
        let mut before_each = vec![];
        let mut seeds = vec![];
        let mut tests = vec![];
        let mut description = None;

        for head in heads {
            match head.heading.children.stringify().trim() {
                "\\--after-all--" => {
                    let heads = head.nodes.into_iter().skip(1).fold(vec![], fold_into_heads);
                    for head in heads {
                        let seed = Seed::from_mdast(head.nodes)?;
                        after_all.push(seed);
                    }
                }
                "\\--after-each--" => {
                    let heads = head.nodes.into_iter().skip(1).fold(vec![], fold_into_heads);
                    for head in heads {
                        let seed = Seed::from_mdast(head.nodes)?;
                        after_each.push(seed);
                    }
                }
                "\\--before-all--" => {
                    let heads = head.nodes.into_iter().skip(1).fold(vec![], fold_into_heads);
                    for head in heads {
                        let seed = Seed::from_mdast(head.nodes)?;
                        before_all.push(seed);
                    }
                }
                "\\--before-each--" => {
                    let heads = head.nodes.into_iter().skip(1).fold(vec![], fold_into_heads);
                    for head in heads {
                        let seed = Seed::from_mdast(head.nodes)?;
                        before_each.push(seed);
                    }
                }
                "\\--description--" => {
                    description = Some(
                        head.nodes
                            .iter()
                            .skip(1)
                            .collect::<Vec<&Node>>()
                            .stringify(),
                    );
                }
                "\\--hints--" => {
                    let heads = head.nodes.into_iter().skip(1).fold(vec![], fold_into_heads);
                    for head in heads {
                        let hint = Hint::from_mdast(head.nodes)?;
                        hints.push(hint);
                    }
                }
                "\\--seed--" => {
                    let heads = head.nodes.into_iter().skip(1).fold(vec![], fold_into_heads);
                    for head in heads {
                        let seed = Seed::from_mdast(head.nodes)?;
                        seeds.push(seed);
                    }
                }
                "\\--tests--" => {
                    let chunks = head.nodes.into_iter().skip(1).chunks(2);

                    let mut id = 0;
                    for chunk in &chunks {
                        let nodes = chunk.collect();
                        let mut test = Test::from_mdast(nodes)?;
                        test.id = id;
                        tests.push(test);
                        id += 1;
                    }
                }
                _ => {
                    return Err(ParseError::BadNode(head.heading.children.stringify()));
                }
            }
        }

        let description =
            description.context(&format!("### --description-- is missing in lesson ${id}"))?;

        let lesson = Lesson {
            after_all,
            after_each,
            before_all,
            before_each,
            hints,
            seeds,
            tests,
            description,
            id,
        };

        Ok(lesson)
    }
}

// Create map between heading and nodes: ("--<heading>--", Vec<Node>)

#[derive(Debug)]
struct Head {
    pub heading: Heading,
    pub nodes: Vec<Node>,
}

fn fold_into_heads(mut acc: Vec<Head>, node: Node) -> Vec<Head> {
    match &node {
        Node::Heading(h) => {
            let heading_text = h.children.stringify();
            let heading_text = heading_text.trim();

            let depth = acc.first().map(|h| h.heading.depth);
            if (depth.is_none() || h.depth == depth.unwrap())
                && heading_text.starts_with("\\--")
                && heading_text.ends_with("--")
            {
                acc.push(Head {
                    heading: h.clone(),
                    nodes: vec![node],
                });
            } else {
                acc.last_mut().unwrap().nodes.push(node);
            }
        }
        _ => {
            acc.last_mut().unwrap().nodes.push(node);
        }
    }
    acc
}

impl FromMdast for Test {
    fn from_mdast(nodes: Vec<Node>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let text = nodes
            .get(0)
            .as_paragraph()
            .context("missing test text")?
            .children
            .stringify();

        let code_node = nodes.get(1).as_code().context("missing test code")?;
        let runner = code_node
            .lang
            .context("missing code lang")?
            .split("runner=")
            .nth(1)
            .context(&format!("missing runner: {:?}", nodes.stringify()))?
            .to_string();
        let code = code_node.value;

        let test = Test {
            runner,
            text,
            code,
            id: 0,
        };

        Ok(test)
    }
}

impl FromMdast for Seed {
    fn from_mdast(node: Vec<Node>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let heading = node.get(0).as_heading().context("missing seed heading")?;

        let heading_text = heading.children.stringify();
        let heading_text = heading_text.trim();

        let seed = if heading_text == "\\--cmd--" {
            let code_node = node.get(1).as_code().unwrap();
            let seed = Seed::Command {
                runner: code_node.lang.clone().unwrap().into(),
                code: code_node.value.clone(),
            };
            seed
        } else if let Some(path) = heading_text
            .strip_prefix("\\--")
            .unwrap()
            .strip_suffix("--")
        {
            let path = PathBuf::from_str(path).unwrap();

            let content = node.get(1).as_code().unwrap().value.clone();

            let seed = Seed::File { path, content };
            seed
        } else {
            return Err(ParseError::BadNode(heading_text.to_string()));
        };

        Ok(seed)
    }
}

impl FromMdast for Hint {
    fn from_mdast(node: Vec<Node>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let heading = node.get(0).as_heading().context("missing hint heading")?;
        let heading_text = heading.children.stringify();
        let heading_text = heading_text
            .trim()
            .strip_prefix("\\--")
            .unwrap()
            .strip_suffix("--")
            .unwrap();

        let id = heading_text
            .parse()
            .expect(format!("hint id is not a number: {}", heading_text).as_str());

        let text = node.into_iter().skip(1).collect::<Vec<Node>>().stringify();

        let hint = Hint { id, text };
        Ok(hint)
    }
}
