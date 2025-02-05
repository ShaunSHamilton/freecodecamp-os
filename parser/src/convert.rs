use std::{iter::Cloned, path::PathBuf, str::FromStr, vec::IntoIter};

use config::{Hook, Lesson, LessonMeta, Project, ProjectMeta, Seed, Test};
use itertools::Itertools;
use markdown::mdast::Node;
use serde_json::Value;

use crate::{
    node_ext::{NodeExt, Utils},
    Context, ParseError,
};

pub trait FromMdast {
    fn from_mdast(node: &Node) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl FromMdast for Project {
    fn from_mdast(node: &Node) -> Result<Self, ParseError> {
        let mut it = node.children().unwrap().iter();
        let project_meta: ProjectMeta = {
            let title = it
                .next()
                .as_heading()
                .context("# <TITLE>")?
                .children
                .stringify();
            // let title = heading.children.stringify();

            let code = it.next().as_code().context("project meta missing")?;
            let json: Value = serde_json::from_str(&code.value).unwrap();

            let id: &Value = json.get("id").context("id missing")?;
            let id = id.as_u64().context("id incorrect type")?.clone();
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
                .collect::<Vec<&Node>>()
                .stringify();
            ProjectMeta {
                title,
                description,
                id: id as usize,
                is_public,
            }
        };

        let mut lessons = vec![];
        // Group nodes by lesson
        let c = it.fold(vec![], |mut acc, node| {
            match node {
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

        for lesson_nodes in c {
            let mut nodes = lesson_nodes.into_iter();

            let id = nodes
                .next()
                .as_heading()
                .context("lesson id")?
                .children
                .stringify()
                .trim()
                .parse::<usize>()
                .unwrap();

            // Split lesson up into chunks of headings
            let mut description = None;
            let mut tests = vec![];
            let mut seeds = vec![];
            let mut before_all = vec![];
            let mut before_each = vec![];
            let mut after_all = vec![];
            let mut after_each = vec![];

            let chunks = nodes.fold(vec![], |mut acc, node| {
                match node {
                    Node::Heading(h) => {
                        let heading_text = h.children.stringify();
                        let heading_text = heading_text.trim();

                        if h.depth == 3
                            && heading_text.starts_with("\\--")
                            && heading_text.ends_with("--")
                        {
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

            for chunk in chunks {
                let mut chunk = chunk.into_iter().cloned();
                let heading = chunk.next().as_heading().context("chunk heading")?;
                let heading_text = heading.children.stringify();
                let heading_text = heading_text.trim();
                match heading_text {
                    "\\--description--" => {
                        description = Some(chunk.collect::<Vec<Node>>().stringify());
                    }
                    "\\--tests--" => {
                        // Generate chunks of `tests_nodes` in twos where (Paragraph(p), Code(c))
                        let tests_chunks = chunk.collect::<Vec<Node>>();
                        let tests_chunks = tests_chunks.chunks(2);
                        for (id, chunk) in tests_chunks.enumerate() {
                            let text_node = chunk
                                .get(0)
                                .as_paragraph()
                                .unwrap()
                                .children
                                .get(0)
                                .as_text()
                                .unwrap();
                            let code_node = chunk.get(1).as_code().unwrap();

                            let test = Test {
                                runner: code_node.lang.clone().unwrap().into(),
                                id,
                                text: text_node.value.clone(),
                                code: code_node.value.clone(),
                            };

                            tests.push(test);
                        }
                    }
                    "\\--seed--" => {
                        seeds = chunk_to_seeds(chunk)?;
                    }
                    "\\--before-all--" => {
                        before_all = chunk_to_seeds(chunk)?;
                    }
                    "\\--before-each--" => {
                        before_each = chunk_to_seeds(chunk)?;
                    }
                    "\\--after-all--" => {
                        after_all = chunk_to_seeds(chunk)?;
                    }
                    "\\--after-each--" => {
                        after_each = chunk_to_seeds(chunk)?;
                    }
                    _ => {
                        return Err(ParseError::BadNode(heading_text.to_string()));
                    }
                }
            }

            let description = description.context("### --description-- is required")?;

            let meta = LessonMeta { description, id };

            let hooks = Hook {
                before_all,
                before_each,
                after_all,
                after_each,
            };

            let lesson = Lesson {
                meta,
                tests,
                seeds,
                hooks,
            };
            lessons.push(lesson);
        }

        let project = Project {
            meta: project_meta,
            lessons,
        };

        Ok(project)
    }
}

fn chunk_to_seeds(chunk: Cloned<IntoIter<&Node>>) -> Result<Vec<Seed>, ParseError> {
    let chunk = chunk.collect::<Vec<Node>>();
    let seed_chunks = chunk.chunks(2);

    let mut seeds = vec![];
    for chunk in seed_chunks {
        let heading = chunk.get(0).as_heading().unwrap();
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
