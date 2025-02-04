use config::{FreeCodeCampConf, Lesson, Project, State};
use parser::parse_project;

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

pub fn set_state(new_state: State) {
    let state_str = serde_json::to_string(&new_state).unwrap();
    std::fs::write("./example/state.json", state_str).unwrap();
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
            let project = parse_project(&file).unwrap();
            projects.push(project);
        }
    }

    projects
}

pub fn read_lesson(project_id: usize, lesson_id: usize) -> Lesson {
    let curriculum_dir = std::fs::read_dir("./example/curriculum").unwrap();

    let project = curriculum_dir.into_iter().find_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let file = std::fs::read_to_string(&path).unwrap();

            let project = parse_project(&file).unwrap();

            if project.meta.id == project_id {
                return Some(project);
            }
        }
        None
    });

    let lesson = project
        .unwrap()
        .lessons
        .into_iter()
        .find(|l| l.meta.id == lesson_id)
        .unwrap();

    lesson
}
