use std::error::Error;

pub trait Plugin {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, command: &str) -> Result<String, Box<dyn Error>>;
    fn can_handle(&self, command: &str) -> bool;
}