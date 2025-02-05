use parser::parse_project;

#[test]
fn markdown() {
    let s = get_markdown();

    let project = parse_project(&s).unwrap();

    assert_eq!(project.title, "Learn freeCodeCampOS\n");
    assert_eq!(project.id, 0);
    assert_eq!(project.description, "In this course, you will learn how to use the `@freecodecamp/freecodecamp-os` package to develop courses.\n");
    assert_eq!(project.is_public, true);
    assert_eq!(project.lessons.len(), 27);

    for (i, lesson) in project.lessons.iter().enumerate() {
        assert_eq!(lesson.id, i);
        assert!(lesson.tests.len() > 0, "Lesson {} has no tests", i);
    }
}

fn get_markdown() -> String {
    let s = std::fs::read_to_string("../example/curriculum/project-1.md").unwrap();

    s
}
