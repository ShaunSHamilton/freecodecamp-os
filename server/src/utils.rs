use config::{FreeCodeCampConf, Project};

pub fn read_config() -> FreeCodeCampConf {
    let config_file = std::fs::read_to_string("./example/freecodecamp.conf.json").unwrap();

    let config: FreeCodeCampConf = serde_json::from_str(&config_file).unwrap();

    config
}

pub fn read_projects() -> Vec<Project> {
    let projects_file = std::fs::read_to_string("./example/projects.json").unwrap();

    let projects: Vec<Project> = serde_json::from_str(&projects_file).unwrap();

    projects
}
