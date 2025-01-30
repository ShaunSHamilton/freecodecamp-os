use config::{Lesson, Project};
use markdown::mdast::Node;

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
    fn get_heading(&self, depth: u8, text: &str) -> Option<Vec<&Node>>;
    fn stringifiy(&self) -> String;
}

impl Utils for Vec<&Node> {
    fn get_heading(&self, depth: u8, text: &str) -> Option<Vec<&Node>> {
        let mut heading_nodes: Vec<&Node> = vec![];
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

    fn stringifiy(&self) -> String {
        self.iter()
            .map(|node| match node {
                Node::Text(t) => t.value.clone(),
                Node::InlineCode(c) => format!("`{}`", c.value),
                _ => "".to_string(),
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Parser for MarkdownParser {
    fn get_project_meta(&self) -> Result<Project, ParserError> {
        let code_node = self.ast.children().unwrap().iter().find(|node| {
            if let Node::Code(_) = node {
                true
            } else {
                false
            }
        });

        if let Some(Node::Code(code)) = code_node {
            if code.lang != Some("json".to_string()) {
                let position = code.position.as_ref().unwrap();
                return Err(ParserError {
                    message: "Project meta not found".to_string(),
                    location: Location {
                        start: position.start.line,
                        end: Some(position.end.line),
                    },
                });
            }
            let project: Project = serde_json::from_str(&code.value).unwrap();
            Ok(project)
        } else {
            Err(ParserError {
                message: "Project meta not found".to_string(),
                location: Location {
                    start: 0,
                    end: None,
                },
            })
        }
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
                lesson_nodes.push(node);
            }
        }

        let description_nodes = lesson_nodes.get_heading(3, "--description--").unwrap();
        let description = description_nodes
            .iter()
            // Skip first node as it is the heading itself
            .skip(1)
            .map(|node| node.stringifiy())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", description);

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
    use markdown::{to_mdast, ParseOptions};

    #[test]
    fn test_get_project_meta() {
        let markdown = r#"
# Project Meta

```json
{
    "id": 1,
    "dashed_name": "project-1"
}
```
"#;
        let ast = to_mdast(markdown, &ParseOptions::default()).unwrap();
        let parser = MarkdownParser { ast };
        let project = parser.get_project_meta().unwrap();
        assert_eq!(project.id, 1);
        assert_eq!(project.dashed_name, "project-1");
    }
}
