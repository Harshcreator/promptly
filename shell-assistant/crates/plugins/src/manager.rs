use crate::traits::{CommandResult, Plugin};
use std::sync::Arc;

/// Plugin manager that maintains a registry of plugins and handles dispatching
pub struct PluginManager {
    plugins: Vec<Arc<dyn Plugin + Send + Sync>>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        PluginManager { plugins: Vec::new() }
    }

    /// Register a plugin with the manager
    pub fn register_plugin<P>(&mut self, plugin: P)
    where
        P: Plugin + Send + Sync + 'static,
    {
        self.plugins.push(Arc::new(plugin));
    }

    /// Process input through all registered plugins
    /// Returns the first matching result, or None if no plugin can handle the input
    pub fn process(&self, input: &str) -> Option<CommandResult> {
        for plugin in &self.plugins {
            if plugin.can_handle(input) {
                if let Some(result) = plugin.handle(input) {
                    return Some(result);
                }
            }
        }
        None
    }

    /// Get a reference to a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&(dyn Plugin + Send + Sync)> {
        self.plugins
            .iter()
            .find(|p| p.name().to_lowercase() == name.to_lowercase())
            .map(|p| p.as_ref())
    }

    /// Get a list of all registered plugins
    pub fn list_plugins(&self) -> Vec<(&str, &str)> {
        self.plugins.iter().map(|p| (p.name(), p.description())).collect()
    }

    /// Get the number of registered plugins
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}
