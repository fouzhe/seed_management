extern crate rand;
pub mod config;
pub mod seed_generator;

pub mod input_seed;
use input_seed::InputSeed;

mod seed_pool;

use seed_generator::SeedGenerator;
use seed_pool::SeedPool;


#[derive(Debug)]
pub struct SeedManagement {
    seed_generator: SeedGenerator,
    seed_pool: SeedPool,
    in_mutate: bool,    
}

impl SeedManagement {
    pub fn new(path:& str)->SeedManagement {
        SeedManagement {
            seed_generator:SeedGenerator::new(),
            seed_pool:SeedPool::new(path),
            in_mutate:false,
        }
    }

    pub fn get_a_seed(&mut self)->Vec<u8> {
        //InputSeed::emptyNew();
        if !self.in_mutate {
            
            let &mut seed_pool: &seed_pool::SeedPool = self.seed_pool;
            let seed = seed_pool.get_a_ini_seed();
            if seed.is_none() {
                self.in_mutate = true;
                let seed_to_mutate = seed_pool.get_a_seed_to_mutate();
                let seed_vec_option = self.seed_generator.get_a_mutated_seed(&seed_to_mutate);
                return seed_vec_option.unwrap();
            }
            seed.unwrap().get_seed_vec()
        }
        else {
            let mut seed_to_mutate = self.seed_pool.get_a_seed_to_mutate();
            let mut seed_vec_option = self.seed_generator.get_a_mutated_seed(&seed_to_mutate);
            while seed_vec_option.is_none() {
                self.seed_pool.seed_index_move();
                seed_to_mutate = self.seed_pool.get_a_seed_to_mutate();
                seed_vec_option = self.seed_generator.get_a_mutated_seed(&seed_to_mutate);
            }
            seed_vec_option.unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
