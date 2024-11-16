
use super::block_property::Property;
use super::block_property::BlockProperty;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockState {
    block_name: String,
    sorted_properties: Vec<BlockProperty>,
}

impl BlockState {
    pub fn new<S: Into<String>, It: IntoIterator<Item = BlockProperty>>(block_name: S, it: It) -> Self {
        let block_name: String = block_name.into();
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

    #[inline]
    pub fn properties(&self) -> &[BlockProperty] {
        &self.sorted_properties
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

/// # Usage
/// You could also replace `block_name` with `"block_name"`. Either is valid. Or even `(block_name)` if `block_name` is a variable.
/// ```rust
/// let state = blockstate!(block_name[
///     facing=Cardinal::West,
///     locked=false,
///     color=Rgb::new(255, 0, 255),
///     "password"=b"hunter2",
///     // If you would like to use a variable for the key, wrap it in parentheses.
///     (some_value)="Hello, world!",
/// ]);
/// ```
/// This macro will not identify identifiers as variables.
/// Instead, they will be converted to strings.
#[macro_export]
macro_rules! blockstate {
    (@token_to_string: $token:ident) => {
        stringify!($token)
    };
    (@token_to_string: $token:literal) => {
        $token
    };
    (@token_to_string: $token:expr) => {
        ($token).to_string()
    };
    ($id:tt $([ $($name:tt = $value:expr),* $(,)? ])?) => {
        $crate::voxel::block::block_state::BlockState::new(
            $crate::blockstate!(@token_to_string: $id),
            [
                $($(
                    $crate::voxel::block::block_property::BlockProperty::new(
                        $crate::blockstate!(@token_to_string: $name),
                        $value
                    ),
                )*)?
            ]
        )
    };
}

pub use crate::blockstate;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn blockstate_test() {
        use crate::voxel::cardinal::Cardinal;
        use crate::rendering::color::*;
        let air = blockstate!(air);
        assert_eq!(air.name(), "air");
        assert!(air.properties().is_empty());
        let facing = "facing";
        let state = blockstate!("chest"[
            (facing)=Cardinal::West,
            locked=false,
            color=Rgb::new(255, 0, 255),
            password=b"hunter2",
        ]);
        assert_eq!(state.properties().len(), 4);
        assert_eq!(state.name(), "chest");

        if let Property::Cardinal(face) = state["facing"] {
            assert_eq!(face, Cardinal::West);
        } else {
            panic!("Property was not Cardinal.");
        }
        if let Property::Cardinal(face) = state["facing"] {
            assert_eq!(face, Cardinal::West);
        } else {
            panic!("Property was not Cardinal.");
        }
        if let Property::Bool(locked) = state["locked"] {
            assert_eq!(locked, false);
        } else {
            panic!("Property was not Bool.");
        }
        if let Property::Bool(locked) = state["locked"] {
            assert_eq!(locked, false);
        } else {
            panic!("Property was not Bool.");
        }
        if let Property::Rgb(color) = state["color"] {
            assert_eq!(color, Rgb::new(255, 0, 255));
        } else {
            panic!("Property was not Rgb.");
        }
        if let Property::Bytes(password) = &state["password"] {
            assert_eq!(password, b"hunter2");
        } else {
            panic!("Property was not Bytes.");
        }

        let state = BlockState::new("test", [
            BlockProperty::new("int", 1234),
            BlockProperty::new("bool", true),
            BlockProperty::new("string", "Hello, world!")
        ]);
        assert_eq!(state.name(), "test");
        assert_eq!(state.properties().len(), 3);
        if let Property::Int(value) = &state["int"] {
            assert_eq!(*value, 1234);
        } else {
            panic!("Property was not Int.");
        }
        if let Property::String(value) = &state["string"] {
            assert_eq!(value, "Hello, world!");
        } else {
            panic!("Property was not String.");
        }
        if let Property::Bool(value) = &state["bool"] {
            assert_eq!(*value, true);
        } else {
            panic!("Property was not Bool.");
        }
    }
}