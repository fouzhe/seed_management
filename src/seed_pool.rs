use super::input_seed;
use input_seed::InputSeed;

#[derive(Debug)]
pub struct SeedPool {
    seed_pool: Vec<InputSeed>,
    seed_index: usize,
}

impl SeedPool {
    pub fn new(path: &str)->SeedPool {
        let seed1 = InputSeed::new(vec![40u8,32u8]);
        let seed2 = InputSeed::new(vec![40u8,32u8]);
        SeedPool {
            seed_pool:vec![seed1, seed2],
            seed_index:0,
        }
    }
    pub fn get_a_ini_seed(&mut self)->Option<&InputSeed> {
        let seed = self.seed_pool.get(self.seed_index);
        self.seed_index += 1;
        if seed.is_none() {
            self.seed_index = 0;
        }
        seed
    }

    pub fn seed_index_move(&mut self) {
        if self.seed_index + 1 == self.seed_pool.len() {
            self.seed_index = 0; 
        }
        else {
           self.seed_index += 1;
        }
    }

    pub fn get_a_seed_to_mutate(&mut self)->InputSeed {
        self.seed_pool[self.seed_index].clone()
    }
}