use config::{FreeCodeCampConf, Lesson, Project, State};
use parser::{MarkdownParser, Parser};

pub fn read_config() -> FreeCodeCampConf {
    let config_file = std::fs::read_to_string("./example/freecodecamp.conf.json").unwrap();

    let config: FreeCodeCampConf = serde_json::from_str(&config_file).unwrap();

    config
}

pub fn read_state() -> State {
    let state_file = std::fs::read_to_string("./example/state.json").unwrap();

    let state: State = serde_json::from_str(&state_file).unwrap();

    state
}

/// Reads all the `.md` files in the `curriculum/` directory, parses them using the `MarkdownParser`, and returns a vector of `Project` structs.
pub fn read_projects() -> Vec<Project> {
    let curriculum_dir = std::fs::read_dir("./example/curriculum").unwrap();

    let mut projects = vec![];
    for entry in curriculum_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let file = std::fs::read_to_string(&path).unwrap();
            let markdown = MarkdownParser::new(&file);

            let project = markdown.get_project_meta().unwrap();
            println!("{:?}", project);
            projects.push(project);
        }
    }

    projects
}

pub fn read_lesson(project_id: u16, lesson_id: u16) -> Lesson {
    let curriculum_dir = std::fs::read_dir("./example/curriculum").unwrap();

    let project_str = curriculum_dir.into_iter().find_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let file = std::fs::read_to_string(&path).unwrap();
            let markdown = MarkdownParser::new(&file);

            let project = markdown.get_project_meta().unwrap();

            if project.meta.id == project_id {
                return Some(markdown);
            }
        }
        None
    });

    let lesson = project_str.unwrap().get_lesson(lesson_id).unwrap();

    lesson
}
