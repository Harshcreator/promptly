pub mod traits;
pub mod git;
pub mod docker;
pub mod manager;

pub use traits::{Plugin, CommandResult};
pub use git::GitPlugin;
pub use docker::DockerPlugin;
pub use manager::PluginManager;