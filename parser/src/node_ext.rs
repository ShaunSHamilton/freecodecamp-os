use markdown::mdast::{Code, Heading, Node, Paragraph};
use mdast_util_to_markdown::to_markdown;

pub trait NodeExt {
    fn as_heading(self) -> Option<Heading>;
    fn as_paragraph(self) -> Option<Paragraph>;
    fn as_code(self) -> Option<Code>;
}

impl NodeExt for Option<Node> {
    fn as_heading(self) -> Option<Heading> {
        match self {
            Some(Node::Heading(heading)) => Some(heading),
            _ => None,
        }
    }

    fn as_paragraph(self) -> Option<Paragraph> {
        match self {
            Some(Node::Paragraph(paragraph)) => Some(paragraph),
            _ => None,
        }
    }

    fn as_code(self) -> Option<Code> {
        match self {
            Some(Node::Code(code)) => Some(code),
            _ => None,
        }
    }
}
impl NodeExt for Option<&Node> {
    fn as_heading(self) -> Option<Heading> {
        match self {
            Some(Node::Heading(heading)) => Some(heading.clone()),
            _ => None,
        }
    }

    fn as_paragraph(self) -> Option<Paragraph> {
        match self {
            Some(Node::Paragraph(paragraph)) => Some(paragraph.clone()),
            _ => None,
        }
    }

    fn as_code(self) -> Option<Code> {
        match self {
            Some(Node::Code(code)) => Some(code.clone()),
            _ => None,
        }
    }
}
impl NodeExt for Option<&mut Node> {
    fn as_heading(self) -> Option<Heading> {
        match self {
            Some(Node::Heading(ref heading)) => Some(heading.clone()),
            _ => None,
        }
    }

    fn as_paragraph(self) -> Option<Paragraph> {
        match self {
            Some(Node::Paragraph(paragraph)) => Some(paragraph.clone()),
            _ => None,
        }
    }

    fn as_code(self) -> Option<Code> {
        match self {
            Some(Node::Code(ref code)) => Some(code.clone()),
            _ => None,
        }
    }
}

pub trait Utils {
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
impl Utils for Vec<&mut Node> {
    fn stringify(&self) -> String {
        self.iter()
            .map(|node| to_markdown(node).unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
impl Utils for Vec<Node> {
    fn stringify(&self) -> String {
        self.iter()
            .map(|node| to_markdown(node).unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
