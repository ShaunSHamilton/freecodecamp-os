use parser::{MarkdownParser, Parser};

#[test]
fn markdown() {
    let s = get_markdown();
    let markdown = MarkdownParser::new(&s);

    let project_meta = markdown.get_project_meta().unwrap();
    let lesson = markdown.get_lesson(2).unwrap();

    println!("{:?}", project_meta);
    println!("{:?}", lesson);
}

fn get_markdown() -> String {
    let s = std::fs::read_to_string("../example/curriculum/project-1.md").unwrap();

    s
}
