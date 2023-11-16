use std::{fs, io};

use crate::config::general::GeneralConfig;

pub fn new_project(
    project_name: Option<String>,
    config: GeneralConfig,
) -> io::Result<Option<String>> {
    if let Some(project_name) = project_name {
        let project_dir = dirs::home_dir()
            .expect("Failed to get home directory")
            .join(config.projects_dir())
            .join(&project_name);

        fs::create_dir_all(project_dir)?;
        return Ok(Some(project_name));
    }

    Ok(None)
}
