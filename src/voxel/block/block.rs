use std::any::Any;

use super::blockregistry::BlockRegistry;

pub trait BlockBehavior: Any {
    // Details
    fn name(&self) -> &str;
    fn display_name(&self) -> Option<&str> { None }
    fn description(&self) -> Option<&str> { None }

    // Callbacks
    #[allow(unused)]
    fn on_register(&self, registry: &BlockRegistry) {}
    
}