use super::config;
use super::input_seed;
use rand;
use rand::Rng;

#[derive(Debug)]
pub struct Selector{

}

impl Selector {

    pub fn whether_select(&self, seed:& input_seed::InputSeed)->bool {
        let probability = rand::thread_rng().gen_range(0, 100);
        if (seed.was_fuzzed || !seed.is_favored) &&  (probability < config::SKIP_TO_NEW_PROB) {
            return false;
        } 
        // To do: add seed_count, dumb_mode, cycle information for selection
        true
    }
}