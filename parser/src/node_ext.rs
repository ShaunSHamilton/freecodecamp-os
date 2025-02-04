use markdown::mdast::{Code, Heading, Node, Paragraph, Text};

pub(crate) trait NodeExt {
    fn as_heading(&self) -> Option<&Heading>;
    fn as_paragraph(&self) -> Option<&Paragraph>;
    fn as_code(&self) -> Option<&Code>;
    fn as_text(&self) -> Option<&Text>;
}

impl NodeExt for Node {
    fn as_heading(&self) -> Option<&Heading> {
        match *self {
            Node::Heading(ref heading) => Some(heading),
            _ => None,
        }
    }

    fn as_paragraph(&self) -> Option<&Paragraph> {
        match *self {
            Node::Paragraph(ref paragraph) => Some(paragraph),
            _ => None,
        }
    }

    fn as_code(&self) -> Option<&Code> {
        match *self {
            Node::Code(ref code) => Some(code),
            _ => None,
        }
    }

    fn as_text(&self) -> Option<&Text> {
        match *self {
            Node::Text(ref text) => Some(text),
            _ => None,
        }
    }
}
