use crate::traits::{CommandResult, Plugin};

pub struct DockerPlugin;

impl Default for DockerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerPlugin {
    pub fn new() -> Self {
        DockerPlugin
    }
}

impl Plugin for DockerPlugin {
    fn name(&self) -> &str {
        "docker"
    }

    fn description(&self) -> &str {
        "Provides Docker command functionality"
    }

    fn can_handle(&self, input: &str) -> bool {
        input.to_lowercase().contains("docker")
            || input.to_lowercase().contains("container")
            || input.to_lowercase().contains("image")
            || input.to_lowercase().contains("volume")
            || input.to_lowercase().contains("compose")
    }

    fn handle(&self, input: &str) -> Option<CommandResult> {
        if !self.can_handle(input) {
            return None;
        }

        let input_lower = input.to_lowercase();

        // Container operations
        if input_lower.contains("list")
            && (input_lower.contains("container") || input_lower.contains("all container"))
        {
            let show_all = input_lower.contains("all");
            let cmd = if show_all { "docker ps -a" } else { "docker ps" };
            let explanation = if show_all {
                "Lists all containers, including stopped ones."
            } else {
                "Lists running containers."
            };

            return Some(CommandResult {
                command: cmd.to_string(),
                explanation: explanation.to_string(),
                executed: false,
                output: None,
            });
        }

        // Image operations
        if input_lower.contains("list") && input_lower.contains("image") {
            return Some(CommandResult {
                command: "docker images".to_string(),
                explanation: "Lists all available Docker images.".to_string(),
                executed: false,
                output: None,
            });
        }

        if input_lower.contains("pull") && input_lower.contains("image") {
            if let Some(image) = extract_image_name(input) {
                return Some(CommandResult {
                    command: format!("docker pull {}", image),
                    explanation: format!("Pulls the Docker image '{}'.", image),
                    executed: false,
                    output: None,
                });
            }

            return Some(CommandResult {
                command: "docker pull ".to_string(),
                explanation: "Pulls a Docker image. You'll need to specify the image name."
                    .to_string(),
                executed: false,
                output: None,
            });
        }

        // Running containers
        if (input_lower.contains("run") || input_lower.contains("start"))
            && (input_lower.contains("container") || input_lower.contains("image"))
        {
            if let Some(image) = extract_image_name(input) {
                return Some(CommandResult {
                    command: format!("docker run {}", image),
                    explanation: format!("Runs a container from the '{}' image.", image),
                    executed: false,
                    output: None,
                });
            }

            return Some(CommandResult {
                command: "docker run ".to_string(),
                explanation: "Runs a Docker container. You'll need to specify the image name."
                    .to_string(),
                executed: false,
                output: None,
            });
        }

        // Stopping containers
        if input_lower.contains("stop") && input_lower.contains("container") {
            if let Some(container) = extract_container_name(input) {
                return Some(CommandResult {
                    command: format!("docker stop {}", container),
                    explanation: format!("Stops the running container '{}'.", container),
                    executed: false,
                    output: None,
                });
            }

            return Some(CommandResult {
                command: "docker stop ".to_string(),
                explanation:
                    "Stops a running container. You'll need to specify the container ID or name."
                        .to_string(),
                executed: false,
                output: None,
            });
        }

        // Removing containers
        if (input_lower.contains("remove") || input_lower.contains("delete"))
            && input_lower.contains("container")
        {
            if let Some(container) = extract_container_name(input) {
                return Some(CommandResult {
                    command: format!("docker rm {}", container),
                    explanation: format!("Removes the container '{}'.", container),
                    executed: false,
                    output: None,
                });
            }

            return Some(CommandResult {
                command: "docker rm ".to_string(),
                explanation:
                    "Removes a container. You'll need to specify the container ID or name."
                        .to_string(),
                executed: false,
                output: None,
            });
        }

        // Removing images
        if (input_lower.contains("remove") || input_lower.contains("delete"))
            && input_lower.contains("image")
        {
            if let Some(image) = extract_image_name(input) {
                return Some(CommandResult {
                    command: format!("docker rmi {}", image),
                    explanation: format!("Removes the image '{}'.", image),
                    executed: false,
                    output: None,
                });
            }

            return Some(CommandResult {
                command: "docker rmi ".to_string(),
                explanation: "Removes a Docker image. You'll need to specify the image ID or name."
                    .to_string(),
                executed: false,
                output: None,
            });
        }

        // Docker compose
        if input_lower.contains("compose") && input_lower.contains("up") {
            return Some(CommandResult {
                command: "docker-compose up".to_string(),
                explanation: "Starts all services defined in docker-compose.yml.".to_string(),
                executed: false,
                output: None,
            });
        }

        if input_lower.contains("compose") && input_lower.contains("down") {
            return Some(CommandResult {
                command: "docker-compose down".to_string(),
                explanation: "Stops and removes all services defined in docker-compose.yml."
                    .to_string(),
                executed: false,
                output: None,
            });
        }

        // Docker build
        if input_lower.contains("build") && input_lower.contains("image") {
            if let Some(tag) = extract_tag(input) {
                return Some(CommandResult {
                    command: format!("docker build -t {} .", tag),
                    explanation: format!("Builds a Docker image with the tag '{}'.", tag),
                    executed: false,
                    output: None,
                });
            }

            return Some(CommandResult {
                command: "docker build -t ".to_string(),
                explanation: "Builds a Docker image. You'll need to specify a tag.".to_string(),
                executed: false,
                output: None,
            });
        }

        // Default fallback for other docker commands
        Some(CommandResult {
            command: "docker ".to_string(),
            explanation: "Docker is a platform for developing, shipping, and running applications in containers.".to_string(),
            executed: false,
            output: None,
        })
    }
}

// Helper functions for extracting information from input
fn extract_image_name(input: &str) -> Option<String> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let idx = words.iter().position(|&w| {
        w.to_lowercase() == "image"
            || w.to_lowercase() == "from"
            || w.to_lowercase() == "called"
            || w.to_lowercase() == "named"
    })?;

    if idx + 1 < words.len() {
        Some(
            words[idx + 1]
                .trim_matches(|c: char| {
                    !c.is_alphanumeric() && c != '.' && c != '_' && c != ':' && c != '/'
                })
                .to_string(),
        )
    } else {
        None
    }
}

fn extract_container_name(input: &str) -> Option<String> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let idx = words.iter().position(|&w| {
        w.to_lowercase() == "container"
            || w.to_lowercase() == "named"
            || w.to_lowercase() == "called"
            || w.to_lowercase() == "id"
    })?;

    if idx + 1 < words.len() {
        Some(
            words[idx + 1]
                .trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '_' && c != '-')
                .to_string(),
        )
    } else {
        None
    }
}

fn extract_tag(input: &str) -> Option<String> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let idx = words.iter().position(|&w| {
        w.to_lowercase() == "tag"
            || w.to_lowercase() == "as"
            || w.to_lowercase() == "name"
            || w.to_lowercase() == "named"
    })?;

    if idx + 1 < words.len() {
        Some(
            words[idx + 1]
                .trim_matches(|c: char| {
                    !c.is_alphanumeric() && c != '.' && c != '_' && c != ':' && c != '/'
                })
                .to_string(),
        )
    } else {
        None
    }
}
