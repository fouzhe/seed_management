extern crate seed_management;
use seed_management::SeedManagement;
use seed_management::config;

extern crate rand;
use rand::Rng;


pub fn run_target(_seed: Vec<u8>)->[u32; 2048] {
    let mut cov = [0;2048];
    let mut rang = rand::thread_rng();
    let tims = rang.gen_range(0, 20); 
    for _i in 0..tims {
        let pos = rang.gen_range(0, 2048); 
        cov[pos] = 1;
    }
    cov
}

fn main() {
    let mut seed_m = SeedManagement::new("124");
    for _i in 0..100 {
        let seed = seed_m.get_a_seed();
        println!("{:?}", seed);
        let cov = run_target(seed);
        // seed_management.deal_cov(seed, cov);

    }

}

