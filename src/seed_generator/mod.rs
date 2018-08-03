/*
   DeepSAFL - seed SeedGenerator
   ------------------------------------------------------

   Written and maintained by Liang Jie <liangjie.mailbox.cn@google.com>
   Copyright 2018. All rights reserved.
*/

mod state_parser;

use self::state_parser::StateParser;
use self::state_parser::FuzzingState;

use super::config;
mod selector;
use self::selector::Selector;

pub mod mutator;
use super::input_seed;

use input_seed::InputSeed;

use rand;

#[derive(Debug)]
pub struct SeedGenerator {
    state_parser: StateParser,
    seed_selector: Selector,
}

impl SeedGenerator {

    pub fn new() -> SeedGenerator{
        SeedGenerator{
            state_parser:StateParser::new(),
            seed_selector: Selector{},
        }
    }

    pub fn origin(input_seed: InputSeed) -> SeedGenerator{
        SeedGenerator{
            state_parser:StateParser::new(),
            seed_selector: Selector{},
        }
    }

    // pub fn change_seed(&mut self, seed_string:String) {
    //     self.seed_string = seed_string.clone();
    // }

    //return Result<U(seed),E>
    pub fn get_a_mutated_seed(&mut self, input_seed: &InputSeed) -> Option<Vec<u8>> { 
        let origin_seed_vec = input_seed.get_seed_vec();
        assert!(origin_seed_vec.len() > 0);
        let seed_len = origin_seed_vec.len() as u64;

        let mut state = self.state_parser.get_next_mutate_state(seed_len);
        println!("{:?}", state);
        //first check if the seed is new and need to select
        if state == FuzzingState::Select {
            if !self.seed_selector.whether_select(input_seed) {
                self.state_parser.change_to_next_state(FuzzingState::Ready);
                println!("{:?}", "not select, try next seed, return E");
                return None;
            }
            // if the seed is selected successfully, change it to next state and mutate it immediately
            println!("{:?}", "select successfully");

            self.state_parser.change_to_next_state(state);
            state = self.state_parser.get_next_mutate_state(seed_len);
            //println!("{:?}", state);
        }

        let mut rang = rand::thread_rng();
        let mut mutated_seed_vec: Vec<u8> = Vec::new();

        match state {
            FuzzingState::Select => {
                println!("{:?}", "I think this would never happen");
            },
            FuzzingState::CalHavocTimes => {
                println!("{:?}", "Congratulations, we calculate the havoc times successfully");
                self.state_parser.change_to_next_state(state);
                return self.get_a_mutated_seed(input_seed);
            },
            FuzzingState::StateFlip1(i) => {
                mutated_seed_vec = mutator::flip_one_bit(&origin_seed_vec,i);
            },
            FuzzingState::StateFlip2(i) => {
                mutated_seed_vec = mutator::flip_two_bits(&origin_seed_vec,i);
            },
            FuzzingState::StateFlip4(i) => {
                mutated_seed_vec = mutator::flip_four_bits(&origin_seed_vec,i);
            },
            FuzzingState::StateFlip8(i) => {
                mutated_seed_vec = mutator::flip_one_byte(&origin_seed_vec,i);
            },
            FuzzingState::StateFlip16(i) => {
                mutated_seed_vec = mutator::flip_two_bytes(&origin_seed_vec,i);
            },
            FuzzingState::StateFlip32(i) => {
                mutated_seed_vec = mutator::flip_four_bytes(&origin_seed_vec,i);
            },
            FuzzingState::StateAddArith8((i,arith_j)) => {
                let mutated_seed_opt = mutator::arithmetic_add_one_byte_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateSubArith8((i,arith_j)) => {
                let mutated_seed_opt = mutator::arithmetic_sub_one_byte_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateAddArith16((i,arith_j)) => {
                let mutated_seed_opt: Option<Vec<u8>> = mutator::arithmetic_add_two_bytes_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateSubArith16((i,arith_j)) => {
                let mutated_seed_opt: Option<Vec<u8>> = mutator::arithmetic_sub_two_bytes_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateAddArith16AnotherEndian((i,arith_j)) => {
                let mutated_seed_opt: Option<Vec<u8>> = mutator::arithmetic_add_two_bytes_another_endian_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateSubArith16AnotherEndian((i,arith_j)) => {
                let mutated_seed_opt: Option<Vec<u8>> = mutator::arithmetic_sub_two_bytes_another_endian_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateAddArith32((i,arith_j)) => {
                let mutated_seed_opt: Option<Vec<u8>> = mutator::arithmetic_add_four_bytes_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateSubArith32((i,arith_j)) => {
                let mutated_seed_opt: Option<Vec<u8>> = mutator::arithmetic_sub_four_bytes_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateAddArith32AnotherEndian((i,arith_j)) => {
                let mutated_seed_opt: Option<Vec<u8>> = mutator::arithmetic_add_four_bytes_another_endian_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateSubArith32AnotherEndian((i,arith_j)) => {
                let mutated_seed_opt: Option<Vec<u8>> = mutator::arithmetic_sub_four_bytes_another_endian_option(&origin_seed_vec,i,arith_j);
                if mutated_seed_opt.is_none() {
                    self.state_parser.change_to_next_state(state);
                    return self.get_a_mutated_seed(input_seed);
                }
                mutated_seed_vec = mutated_seed_opt.unwrap();
            },
            FuzzingState::StateInterest8((i,index_count)) => {
                mutated_seed_vec = mutator::interesting8_replace(&origin_seed_vec,i,index_count);
            },
            FuzzingState::StateInterest16((i,index_count)) => {
                mutated_seed_vec = mutator::interesting16_replace(&origin_seed_vec,i,index_count);
            },
            FuzzingState::StateInterest16AnotherEndian((i,index_count)) => {
                mutated_seed_vec = mutator::interesting16_replace_another_endian(&origin_seed_vec,i,index_count);
            },
            FuzzingState::StateInterest32((i,index_count)) => {
                mutated_seed_vec = mutator::interesting32_replace(&origin_seed_vec,i,index_count);
            },
            FuzzingState::StateInterest32AnotherEndian((i,index_count)) => {
                mutated_seed_vec = mutator::interesting32_replace_another_endian(&origin_seed_vec,i,index_count);
            },
            FuzzingState::StateHavoc((_outer_cnt, _inner_cnt)) => {
                mutated_seed_vec = mutator::havoc_mutate(&origin_seed_vec,&mut rang);
            },
            _=> {
                println!("{:?}", "end");
                return None;
            },
        }
        self.state_parser.change_to_next_state(state);
        return Some(mutated_seed_vec);
    }
}

// impl Iterator for SeedGenerator {
//     type Item = Vec<u8>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let p = self.input_seed.clone();
//         self.get_a_mutated_seed(&p)
//     }
// }


//problems:
//1. when we calculate havoc times, we need some other information
//2. if the output seed is helpful, we need to extend the havoc times
//3. splicing operation needs to subtract other seeds, only one seed can not do splice

//let stateParser=StateParser::new(&origin_seed);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
