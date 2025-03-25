
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Polarity {
    /// Negative
    Negative = -1,
    /// Positive
    Positive = 1,
}

impl Polarity {
    pub fn polarize<T: std::ops::Neg<Output = T>>(self, value: T) -> T {
        match self {
            Polarity::Negative => value.neg(),
            Polarity::Positive => value,
        }
    }

    pub fn set_polarity<T>(self, value: T) -> T
    where T: std::ops::Neg<Output = T> + std::cmp::PartialOrd<T> + Default + Copy {
        let hopefully_zero = T::default();
        match self {
            Polarity::Negative if value < hopefully_zero => value,
            Polarity::Positive if value >= hopefully_zero => value,
            _ => value.neg(),
        }
    }

    pub fn get<T>(value: T) -> Self
    where T: std::cmp::PartialOrd<T> + Default + Copy {
        let hopefully_zero = T::default();
        if value < hopefully_zero {
            Self::Negative
        } else {
            Self::Positive
        }
    }
}

pub trait GetPolarity {
    fn polarity(self) -> Polarity;
}

impl<T: std::cmp::PartialOrd<T> + Default + Copy> GetPolarity for T {
    fn polarity(self) -> Polarity {
        Polarity::get(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn polarity_test() {
        let v = -3.14f32;
        println!("{:?}", v.polarity());
        println!("{}", Polarity::Positive.set_polarity(v));
        println!("{:?}", Polarity::get(0.1f32));
    }
}