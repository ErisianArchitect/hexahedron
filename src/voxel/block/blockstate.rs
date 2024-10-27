use super::blockproperty::BlockProperty;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockState {
    block_name: String,
    sorted_properties: Vec<BlockProperty>,
}

impl BlockState {
    pub fn new<S: AsRef<str>, It: IntoIterator<Item = BlockProperty>>(block_name: S, it: It) -> Self {
        let block_name = block_name.as_ref().to_owned();
        let mut sorted_properties = it.into_iter().collect::<Vec<_>>();
        sorted_properties.sort();
        Self {
            block_name,
            sorted_properties
        }
    }

    pub fn name(&self) -> &str {
        &self.block_name
    }
}