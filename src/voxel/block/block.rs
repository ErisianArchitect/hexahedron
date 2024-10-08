use std::any::Any;

pub trait BlockBehavior: Any {
    // Details
    fn name(&self) -> &str;
    fn display_name(&self) -> Option<&str>;
    fn description(&self) -> Option<&str>;

    // Callbacks
    fn on_register(&self) {}
    
}