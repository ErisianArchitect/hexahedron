
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    Opaque(u16),
    Transparent(u16),
}

impl Layer {
    pub fn level(self) -> u16 {
        match self {
            Layer::Opaque(level) => level,
            Layer::Transparent(level) => level,
        }
    }
}

impl std::cmp::PartialOrd<Layer> for Layer {
    fn partial_cmp(&self, other: &Layer) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Layer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Layer::Opaque(lhs), Layer::Opaque(rhs)) => lhs.cmp(&rhs),
            (Layer::Transparent(lhs), Layer::Transparent(rhs)) => lhs.cmp(rhs),
            (&Layer::Opaque(lhs), &Layer::Transparent(rhs)) => if lhs <= rhs {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            },
            (&Layer::Transparent(lhs), &Layer::Opaque(rhs)) => if lhs >= rhs {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct LayerBuffer {
        opaque: Vec<Layer>,
        transparent: Vec<Layer>,
    }

    fn collect_layers(layers: &[Layer]) -> Vec<LayerBuffer> {
        // First pass, create sorted buffer.
        let mut sorted = layers.iter().cloned().collect::<Vec<_>>();
        sorted.sort();


    }

    #[test]
    fn layer_sort_test() {
        fn t(layer: u16) -> Layer {
            Layer::Transparent(layer)
        }
        fn o(layer: u16) -> Layer {
            Layer::Opaque(layer)
        }

        let mut layers = vec![
            t(1),
            t(3),
            t(8),
            t(4),
            o(3),
            o(4),
            o(1),
            o(8),
        ];
        layers.sort();
        println!("{layers:#?}");
    }
}