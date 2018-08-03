extern crate rand;
pub mod config;
pub mod seed_generator;

pub mod input_seed;
use input_seed::InputSeed;

mod seed_pool;

use seed_generator::SeedGenerator;
use seed_pool::SeedPool;


// #[derive(Debug)]
pub struct SeedManagement {
    seed_generator: SeedGenerator,
    seed_pool: SeedPool,
    in_mutate: bool,
    covered_bit_map: [u32;config::MAP_SIZE],
    seed_current: Vec<u8>,
}

impl SeedManagement {
    pub fn new(path:& str)->SeedManagement {
        SeedManagement {
            seed_generator:SeedGenerator::new(),
            seed_pool:SeedPool::new(path),
            in_mutate:false,
            covered_bit_map:[0;config::MAP_SIZE],
            seed_current: Vec::new(),
        }
    }

    //You get from me
    pub fn get_a_seed(&mut self)->Vec<u8> {
        if !self.in_mutate {
            if let Some(seed) = self.seed_pool.get_a_ini_seed(){
                 self.seed_current = seed.get_seed_vec();
                 return self.seed_current.clone()
            }
            self.in_mutate = true;
            let seed_to_mutate = self.seed_pool.get_a_seed_to_mutate();
            let seed_vec_option = self.seed_generator.get_a_mutated_seed(&seed_to_mutate);
            self.seed_current = seed_vec_option.unwrap();
            return self.seed_current.clone();
        }
        else {
            let mut seed_to_mutate = self.seed_pool.get_a_seed_to_mutate();
            let mut seed_vec_option = self.seed_generator.get_a_mutated_seed(&seed_to_mutate);
            while seed_vec_option.is_none() {
                self.seed_pool.seed_index_move();
                seed_to_mutate = self.seed_pool.get_a_seed_to_mutate();
                seed_vec_option = self.seed_generator.get_a_mutated_seed(&seed_to_mutate);
            }
            self.seed_current = seed_vec_option.unwrap();
            return self.seed_current.clone();
        }
    }

    pub fn has_new_bits_and_change(&mut self, cov_info:[u32; config::MAP_SIZE])->bool {
        let mut is_changed: bool = false;
        for i in 0..config::MAP_SIZE {
            if self.covered_bit_map[i] == 0 && cov_info[i] != 0{
                self.covered_bit_map[i] = 1;
                is_changed = true;
            }
        }
        is_changed
    }

    //You give it to me
    pub fn give_coverage_info(&mut self, cov_info:[u32; config::MAP_SIZE]) {
        if self.has_new_bits_and_change(cov_info) {
            self.seed_pool.push_a_seed(self.seed_current.clone());
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
