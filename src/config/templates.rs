use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct WorkspaceTemplate {
    name: String,
    commands: Vec<String>,
}

impl WorkspaceTemplate {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn commands(&self) -> &[String] {
        self.commands.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{templates::WorkspaceTemplate, WorkflowsConfig};

    #[test]
    fn templates_works() {
        let toml = "\
                    [[template]]\n\
                    name = 'rust'\n\
                    commands = ['cargo init']\n\
                    \n\
                    [[template]]\n\
                    name = 'go'\n\
                    commands = ['go mod init']";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        let wanted = vec![
            WorkspaceTemplate {
                name: "rust".to_string(),
                commands: vec!["cargo init".to_string()],
            },
            WorkspaceTemplate {
                name: "go".to_string(),
                commands: vec!["go mod init".to_string()],
            },
        ];

        assert_eq!(config.templates(), wanted);
    }

    #[test]
    fn blank_templates_works() {
        let toml = "";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.templates(), vec![]);
    }
}
