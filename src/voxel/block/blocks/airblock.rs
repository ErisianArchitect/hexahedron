use super::super::block::BlockBehavior;

pub struct AirBlock;

impl BlockBehavior for AirBlock {
    fn name(&self) -> &str {
        "air"
    }

    fn display_name(&self) -> Option<&str> {
        Some("Air")
    }

    fn description(&self) -> Option<&str> {
        Some("A block of air.")
    }
}