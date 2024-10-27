use std::any::Any;

pub trait BlockBehavior: Any {
    // Details
    fn name(&self) -> &str;
    fn display_name(&self) -> Option<&str> { None }
    fn description(&self) -> Option<&str> { None }

    // Callbacks
    fn on_register(&self) {}
    
}