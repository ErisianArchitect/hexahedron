use deterministic::DeterministicHash;
use rand::prelude::*;
use super::hashing::*;

pub fn seed_rng64<T: DeterministicHash>(source: T) -> StdRng {
    StdRng::seed_from_u64(deterministic::xxhash64(source))
}

pub fn seed_rng256<T: DeterministicHash>(source: T) -> StdRng {
    let mut buffer = [0u8; 32];
    let hash = deterministic::blake3(source);
    buffer.copy_from_slice(hash.as_bytes());
    StdRng::from_seed(buffer)
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
            // let mut rng = ("base_seed", 420, 69).seed_rng();
            let mut rng = seed_rng256(&1i32);
            render(&mut rng);
        }
        hline();
        {
            // let mut rng = ("base_seed", 421, 69).seed_rng();
            render(&mut seed_rng256(("base_seed", 421, 69)));
        }
        hline();
    }
}