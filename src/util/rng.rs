use std::hash::Hash;
use rand::prelude::*;
use super::hashing::HashExt;

pub trait RngSeedSource {
    fn seed_rng(&self) -> StdRng;
}

impl<T: Hash> RngSeedSource for T {
    /// Hashes value using [twox_hash::XxHash64] then returns a [rand::rngs::StdRng] seeded from the hash.
    fn seed_rng(&self) -> StdRng {
        StdRng::seed_from_u64(self.xxhash64())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use rand::distributions::WeightedIndex;

    use super::*;
    
    #[test]
    fn seed_rng_test() {
        let hline = || println!("****************************************************************");
        fn render(rng: &mut StdRng) {
            const TILES: &[char] = &[' ', '+', '*', '#'];
            const WEIGHTS: &[u32] = &[250, 12, 3, 1];

            struct TileGen<'a> {
                index: WeightedIndex<u32>,
                rng: &'a mut StdRng,
            }

            impl<'a> TileGen<'a> {
                fn new(rng: &'a mut StdRng) -> Self {
                    Self {
                        index: WeightedIndex::new(WEIGHTS).unwrap(),
                        rng,
                    }
                }

                fn next(&mut self) -> char {
                    TILES[self.index.sample(self.rng)]
                }
            }
            
            let mut gen = TileGen::new(rng);
            for _y in 0..8 {
                for _x in 0..12 {
                    print!("{}", gen.next());
                }
                println!();
            }
        }
        hline();
        {
            let mut rng = ("base_seed", 420, 69).seed_rng();
            render(&mut rng);
        }
        hline();
        {
            // let mut rng = ("base_seed", 421, 69).seed_rng();
            render(&mut ("base_seed", 421, 69).seed_rng());
        }
        hline();
    }
}