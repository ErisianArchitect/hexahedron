
use super::blockproperty::Property;
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

    #[inline]
    pub fn name(&self) -> &str {
        &self.block_name
    }

    pub fn get_property<S: AsRef<str>>(&self, name: S) -> &Property {
        if let Ok(index) = self.sorted_properties.binary_search_by(|prop| {
            name.as_ref().cmp(&prop.name)
        }) {
            &self.sorted_properties[index].value
        } else {
            &Property::NULL
        }
    }
}

impl<S: AsRef<str>> std::ops::Index<S> for BlockState {
    type Output = Property;
    fn index(&self, index: S) -> &Self::Output {
        self.get_property(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn blockstate_test() {
        let state = BlockState::new("test", [
            BlockProperty::new("first", 1234),
            BlockProperty::new("second", true),
            BlockProperty::new("third", "Hello, world!")
        ]);
        if let Property::String(value) = &state["third"] {
            println!("{}", value);
        } else {
            println!("Failed");
        }
    }
}