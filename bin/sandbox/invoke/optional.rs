use std::hash::Hash;
use std::fmt::Debug;

pub enum Optional<T> {
    None,
    Some(T),
}

impl<T: Clone> Clone for Optional<T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Optional::None => Optional::None,
            Optional::Some(some) => Optional::Some(some.clone()),
        }
    }
}

impl<T: Copy> Copy for Optional<T> {}

impl<T: PartialEq> PartialEq for Optional<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Optional::Some(lhs), Optional::Some(rhs)) => lhs.eq(rhs),
            (Optional::None, Optional::None) => true,
            _ => false,
        }
    }
}

impl<T: Eq> Eq for Optional<T> {}

impl<T: PartialOrd> PartialOrd for Optional<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        Some(match (self, other) {
            (Optional::None, Optional::None) => Equal,
            (Optional::None, Optional::Some(_)) => Less,
            (Optional::Some(_), Optional::None) => Greater,
            (Optional::Some(lhs), Optional::Some(rhs)) => return lhs.partial_cmp(rhs),
        })
    }
}

impl<T: Ord> Ord for Optional<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Optional::None, Optional::None) => std::cmp::Ordering::Equal,
            (Optional::None, Optional::Some(_)) => std::cmp::Ordering::Less,
            (Optional::Some(_), Optional::None) => std::cmp::Ordering::Greater,
            (Optional::Some(lhs), Optional::Some(rhs)) => lhs.cmp(rhs),
        }
    }
}

impl<T: Hash> Hash for Optional<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Optional::None => state.write_u8(0),
            Optional::Some(some) => {
                state.write_u8(1);
                some.hash(state)
            },
        }
    }
}

impl<T: Debug> Debug for Optional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Optional::None => write!(f, "None"),
            Optional::Some(some) => write!(f, "Some({some:?})"),
        }
    }
}

impl<T> Optional<T> {
    // destructor of `Option<T>` cannot be evaluated at compile-time
    #[inline]
    pub fn from_option(option: Option<T>) -> Self {
        match option {
            Some(some) => Optional::Some(some),
            None => Optional::None,
        }
    }

    #[inline]
    pub fn to_option(self) -> Option<T> {
        match self {
            Optional::None => None,
            Optional::Some(some) => Some(some),
        }
    }
}

impl<T> From<Optional<T>> for Option<T> {
    fn from(value: Optional<T>) -> Self {
        match value {
            Optional::None => None,
            Optional::Some(some) => Some(some),
        }
    }
}

impl<T> From<Option<T>> for Optional<T> {
    #[inline]
    fn from(value: Option<T>) -> Self {
        Self::from_option(value)
    }
}