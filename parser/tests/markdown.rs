use parser::parse_project;

#[test]
fn markdown() {
    let s = get_markdown();

    let project = parse_project(&s).unwrap();

    println!("{:?}", project);
}

fn get_markdown() -> String {
    let s = std::fs::read_to_string("../example/curriculum/project-1.md").unwrap();

    s
}
