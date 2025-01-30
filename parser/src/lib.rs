use config::{Lesson, Project, ProjectMeta};
use markdown::mdast::Node;
use mdast_util_to_markdown::to_markdown;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub start: usize,
    pub end: Option<usize>,
}

pub trait Parser {
    fn get_project_meta(&self) -> Result<Project, ParserError>;
    fn get_lesson(&self, lesson_number: u16) -> Result<Lesson, ParserError>;
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nParse Error: Position {}:{:?}",
            self.message, self.location.start, self.location.end
        )
    }
}

pub struct MarkdownParser {
    pub ast: Node,
}

impl MarkdownParser {
    pub fn new(markdown_str: &str) -> Self {
        let ast = markdown::to_mdast(markdown_str, &markdown::ParseOptions::gfm()).unwrap();
        MarkdownParser { ast }
    }
}

trait Utils {
    fn get_heading(&self, depth: u8, text: &str) -> Option<Vec<Node>> {
        unimplemented!()
    }
    fn stringify(&self) -> String;
}

impl Utils for Vec<&Node> {
    fn stringify(&self) -> String {
        self.iter()
            .map(|node| to_markdown(node).unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Utils for Vec<Node> {
    fn get_heading(&self, depth: u8, text: &str) -> Option<Vec<Node>> {
        let mut heading_nodes: Vec<Node> = vec![];
        let mut take = false;
        for node in self {
            if let Node::Heading(heading) = node {
                if heading.depth == depth {
                    let heading_text = heading.children.get(0).unwrap();

                    if let Node::Text(t) = heading_text {
                        if t.value == text {
                            take = true;
                        } else {
                            break;
                        }
                    }
                }
            }

            if take {
                heading_nodes.push(node.clone());
            }
        }

        Some(heading_nodes)
    }

    fn stringify(&self) -> String {
        self.iter()
            .map(|node| to_markdown(node).unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Parser for MarkdownParser {
    fn get_project_meta(&self) -> Result<Project, ParserError> {
        let mut project_meta_nodes = vec![];
        let mut take = false;

        for node in self.ast.children().unwrap() {
            match node {
                Node::Heading(heading) => {
                    if heading.depth == 1 {
                        take = true;
                    } else {
                        take = false;
                        break;
                    }
                }
                _ => {}
            };

            if take {
                project_meta_nodes.push(node.clone());
            }
        }

        let title = project_meta_nodes
            .iter()
            .find(|n| {
                if let Node::Heading(heading) = n {
                    if heading.depth == 1 {
                        return true;
                    }
                }
                false
            })
            .unwrap()
            .to_string();

        let description = project_meta_nodes
            .iter()
            .filter(|n| match n {
                Node::Heading(heading) => {
                    return heading.depth != 1;
                }
                Node::Code(_) => false,
                _ => {
                    return true;
                }
            })
            .collect::<Vec<_>>()
            .stringify();

        let meta = project_meta_nodes
            .iter()
            .find(|n| {
                if let Node::Code(_) = n {
                    return true;
                }
                false
            })
            .unwrap()
            .to_string();
        let meta = serde_json::from_str(&meta).unwrap();

        let project = Project {
            description,
            meta,
            title,
        };

        Ok(project)
    }

    fn get_lesson(&self, lesson_number: u16) -> Result<Lesson, ParserError> {
        // Get all nodes between `## {lesson_number}` and the next `## {lesson_number + 1}` or EOF
        let mut lesson_nodes = vec![];
        let mut take = false;
        for node in self.ast.children().unwrap() {
            if let Node::Heading(heading) = node {
                if heading.depth == 2 {
                    let heading_text = heading.children.get(0).unwrap();

                    if let Ok(num) = heading_text.to_string().trim().parse::<u16>() {
                        if num == lesson_number {
                            take = true;
                        } else if num == lesson_number + 1 {
                            break;
                        }
                    }
                }
            }

            if take {
                lesson_nodes.push(node.clone());
            }
        }

        let description_nodes = lesson_nodes.get_heading(3, "--description--").unwrap();
        let description = description_nodes.stringify();

        let lesson = Lesson {
            description,
            // hooks: todo!(),
            id: lesson_number,
            // meta: todo!(),
            // seed: todo!(),
            // tests: todo!(),
            // title: todo!(),
        };

        Ok(lesson)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_project_meta() {
        let markdown = r#"
# Project Meta

```json
{
    "id": 1,
    "is_public": true
}
```
"#;
        let parser = MarkdownParser::new(markdown);
        let project = parser.get_project_meta().unwrap();
        assert_eq!(project.meta.id, 1);
        assert_eq!(project.meta.is_public, true);
    }
}
