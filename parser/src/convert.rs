use std::{iter::Cloned, path::PathBuf, str::FromStr, vec::IntoIter};

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

        // let project_meta_nodes = it
        //     .peeking_take_while(|node| {
        //         if let Node::Heading(h) = node {
        //             if h.depth == 2 && h.children.stringify().trim().parse::<usize>().is_ok() {
        //                 return false;
        //             }
        //         }
        //         true
        //     })
        //     .collect::<Vec<Node>>();

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

        // // Split lesson up into chunks of headings
        // let mut description = None;
        // let mut tests = vec![];
        // let mut seeds = vec![];
        // let mut hints = vec![];
        // let mut before_all = vec![];
        // let mut before_each = vec![];
        // let mut after_all = vec![];
        // let mut after_each = vec![];

        // let chunks = it.fold(vec![], |mut acc, node| {
        //     match node {
        //         Node::Heading(h) => {
        //             let heading_text = h.children.stringify();
        //             let heading_text = heading_text.trim();

        //             if h.depth == 3
        //                 && heading_text.starts_with("\\--")
        //                 && heading_text.ends_with("--")
        //             {
        //                 acc.push(vec![node]);
        //             } else {
        //                 acc.last_mut().unwrap().push(node);
        //             }
        //         }
        //         _ => {
        //             acc.last_mut().unwrap().push(node);
        //         }
        //     }
        //     acc
        // });

        // for chunk in chunks {
        //     let mut chunk = chunk.into_iter().cloned();
        //     println!("{:?}\n\n", chunk);
        //     let heading = chunk.next().as_heading().context("chunk heading")?;
        //     let heading_text = heading.children.stringify();
        //     let heading_text = heading_text.trim();
        //     match heading_text {
        //         "\\--description--" => {
        //             description = Some(chunk.collect::<Vec<Node>>().stringify());
        //         }
        //         "\\--tests--" => {
        //             // Generate chunks of `tests_nodes` in twos where (Paragraph(p), Code(c))
        //             let tests_chunks = chunk.collect::<Vec<Node>>();
        //             let tests_chunks = tests_chunks.chunks(2);
        //             for (id, chunk) in tests_chunks.enumerate() {
        //                 let text_node = chunk
        //                     .get(0)
        //                     .as_paragraph()
        //                     .unwrap()
        //                     .children
        //                     .get(0)
        //                     .as_text()
        //                     .unwrap();
        //                 let code_node = chunk.get(1).as_code().unwrap();

        //                 let test = Test {
        //                     runner: code_node.lang.clone().unwrap().into(),
        //                     id,
        //                     text: text_node.value.clone(),
        //                     code: code_node.value.clone(),
        //                 };

        //                 tests.push(test);
        //             }
        //         }
        //         "\\--seed--" => {
        //             seeds = chunk_to_seeds(chunk)?;
        //         }
        //         "\\--hints--" => {}
        //         "\\--before-all--" => {
        //             before_all = chunk_to_seeds(chunk)?;
        //         }
        //         "\\--before-each--" => {
        //             before_each = chunk_to_seeds(chunk)?;
        //         }
        //         "\\--after-all--" => {
        //             after_all = chunk_to_seeds(chunk)?;
        //         }
        //         "\\--after-each--" => {
        //             after_each = chunk_to_seeds(chunk)?;
        //         }
        //         _ => {
        //             return Err(ParseError::BadNode(heading_text.to_string()));
        //         }
        //     }
        // }

        // let description = description.context("### --description-- is required")?;
        // let mut it = nodes.into_iter().peekable();
        // let lesson_meta_nodes = it.peeking_take_while(|node| {
        //     if let Node::Heading(h) = node {
        //         let heading_text = h.children.stringify();
        //         let heading_text = heading_text.trim();

        //         if h.depth == 3 && heading_text.starts_with("\\--") && heading_text.ends_with("--")
        //         {
        //             return false;
        //         }
        //     }
        // });
        // let hooks = Hook {
        //     before_all,
        //     before_each,
        //     after_all,
        //     after_each,
        // };

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
                "\\--before-all--" => {
                    // before_all = head.nodes.into_iter().fold(vec![], fold_into_heads);
                }
                "\\--before-each--" => {
                    // before_each = head.nodes.into_iter().fold(vec![], fold_into_heads);
                }
                "\\--after-all--" => {
                    // after_all = head.nodes.into_iter().fold(vec![], fold_into_heads);
                }
                "\\--after-each--" => {
                    // after_each = head.nodes.into_iter().fold(vec![], fold_into_heads);
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
                }
                "\\--seeds--" => {
                    let heads = head.nodes.into_iter().fold(vec![], fold_into_heads);
                }
                "\\--tests--" => {
                    let heads = head.nodes.into_iter().skip(1).fold(vec![], fold_into_heads);
                    println!("{:?}", heads);
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

            if h.depth == 3 && heading_text.starts_with("\\--") && heading_text.ends_with("--") {
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
    fn from_mdast(node: Vec<Node>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl FromMdast for Seed {
    fn from_mdast(node: Vec<Node>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl FromMdast for Hint {
    fn from_mdast(node: Vec<Node>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        todo!()
    }
}

fn chunk_to_seeds(chunk: Cloned<IntoIter<&Node>>) -> Result<Vec<Seed>, ParseError> {
    let chunk = chunk.collect::<Vec<Node>>();
    // println!("{:?}\n\n", chunk);
    let seed_chunks = chunk.chunks(2);

    // println!("{:?}\n\n", seed_chunks);

    let mut seeds = vec![];
    for chunk in seed_chunks {
        let heading = chunk.get(0).as_heading().context(&format!("{:?}", chunk))?;
        let heading_text = heading.children.stringify();
        let heading_text = heading_text.trim();

        let seed = if heading_text == "\\--cmd--" {
            let code_node = chunk.get(1).as_code().unwrap();
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

            let content = chunk.get(1).as_code().unwrap().value.clone();

            let seed = Seed::File { path, content };
            seed
        } else {
            return Err(ParseError::BadNode(heading_text.to_string()));
        };

        seeds.push(seed);
    }

    Ok(seeds)
}
