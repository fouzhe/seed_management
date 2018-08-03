#[derive(Debug, Clone)]
pub struct InputSeed {
    pub is_favored:bool,
    pub was_fuzzed:bool,
    pub seed_vec:Vec<u8>,
}

impl InputSeed {
    pub fn emptyNew()-> InputSeed {
        InputSeed {
            is_favored:true,
            was_fuzzed:false,
            seed_vec:Vec::new(),
        }
    }
    // pub fn new(is_favored:bool, was_fuzzed:bool, seed_vec:Vec<u8>)-> InputSeed {
    //     InputSeed {
    //         is_favored:is_favored,
    //         was_fuzzed:was_fuzzed,
    //         seed_vec:seed_vec,
    //     }

    // }
    pub fn new(seed_vec:Vec<u8>)-> InputSeed {
        InputSeed {
            is_favored:true,
            was_fuzzed:false,
            seed_vec:seed_vec,
        }

    }
    // pub fn set_seed_vec(&mut self, seed_vec:Vec<u8>) {
    //     self.seed_vec = seed_vec;
    // }


    pub fn get_seed_vec(&self)->Vec<u8> {
        self.seed_vec.clone()
    }
}