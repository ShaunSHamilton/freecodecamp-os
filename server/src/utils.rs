use std::{
    fs::{read_dir, read_to_string, ReadDir},
    path::PathBuf,
};

use anyhow::Context;
use config::{FreeCodeCampConf, Lesson, Project, State};
use parser::parse_project;

/// Read `freecodecamp.conf.json` in root
pub fn read_config() -> anyhow::Result<FreeCodeCampConf> {
    // let dir =
    //     read_dir("./").context("unable to search directories for freecodecamp.conf.json file")?;

    // if let Some(path) = recurse_dir_until_file(dir, 0) {
    //     let config_file =
    //         read_to_string(path).context("unable to read freecodecamp.conf.json file")?;
    //     let config: FreeCodeCampConf =
    //         serde_json::from_str(&config_file).context("bad freecodecamp.conf.json format")?;
    //     return Ok(config);
    // }

    let config_file = read_to_string("./freecodecamp.conf.json")
        .context("unable to read freecodecamp.conf.json file in root")?;
    let config: FreeCodeCampConf =
        serde_json::from_str(&config_file).context("bad freecodecamp.conf.json format")?;

    Ok(config)
}

fn _recurse_dir_until_file(dir: ReadDir, depth: usize) -> Option<PathBuf> {
    if depth >= 3 {
        return None;
    }

    for entity in dir {
        if let Ok(entity) = entity {
            if let Ok(file_type) = entity.file_type() {
                let path = entity.path();

                if file_type.is_file() && path.ends_with("freecodecamp.conf.json") {
                    return Some(path);
                }

                if file_type.is_dir() {
                    if let Ok(dir) = read_dir(path) {
                        if let Some(found) = _recurse_dir_until_file(dir, depth + 1) {
                            return Some(found);
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn read_state(config: &FreeCodeCampConf) -> State {
    let state_path = &config.config.state;
    let state_file = std::fs::read_to_string(state_path).unwrap();

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

            if project.id == project_id {
                return Some(project);
            }
        }
        None
    });

    let lesson = project
        .unwrap()
        .lessons
        .into_iter()
        .find(|l| l.id == lesson_id)
        .unwrap();

    lesson
}
