pub mod docker;
pub mod git;
pub mod manager;
pub mod traits;

pub use docker::DockerPlugin;
pub use git::GitPlugin;
pub use manager::PluginManager;
pub use traits::{CommandResult, Plugin};
