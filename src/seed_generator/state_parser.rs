/*
   DeepSAFL - state parser
   ------------------------------------------------------

   Written and maintained by Liang Jie <liangjie.mailbox.cn@google.com>
   Copyright 2018. All rights reserved.
*/

use super::config;
use rand;
use rand::Rng;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum FuzzingState{
          Ready,
          Select,
          CalHavocTimes,
          StateFlip1(u64),
          StateFlip2(u64),
          StateFlip4(u64),
          StateFlip8(u64),
          StateFlip16(u64),
          StateFlip32(u64),
          StateAddArith8((u64,u8)),
          StateSubArith8((u64,u8)),
          StateAddArith16((u64,u16)),
          StateSubArith16((u64,u16)),
          StateAddArith16AnotherEndian((u64,u16)),
          StateSubArith16AnotherEndian((u64,u16)),
          StateAddArith32((u64,u32)),
          StateSubArith32((u64,u32)),
          StateAddArith32AnotherEndian((u64,u32)),
          StateSubArith32AnotherEndian((u64,u32)),
          StateInterest8((u64,u8)),
          StateInterest16((u64,u8)),
          StateInterest16AnotherEndian((u64,u8)),
          StateInterest32((u64,u8)),
          StateInterest32AnotherEndian((u64,u8)),
          StateHavoc((u64,u8)),
          End
}

impl Default for FuzzingState {
    fn default() -> FuzzingState { FuzzingState::Ready }
}


#[derive(Default)]
#[derive(Debug)]
pub struct StateParser{
    seed_len: u64,
    mutate_state: FuzzingState,
    havoc_outer_times: u64,
    havoc_inner_times: u8,
}

impl StateParser {
    pub fn new() -> StateParser {
        StateParser{
            seed_len:0,
            mutate_state:FuzzingState::Ready,
            havoc_outer_times: 0,
            havoc_inner_times: 0,
        }
    }

    fn set_seed_len(&mut self, input_seed_len:u64) {
        self.seed_len = input_seed_len;
    }

    //calculate the havoc times, needs more information like exe_us, bitmap_size, ...
    //power schedule is implemented here

    //Attention: In the initial developing stage, we focus on how to mutate the seed,
    //so we just use a constant number to replace the calculating result,
    //in future we may use a structure to represent the seed with its information and other things
    fn calculate_havoc_outer_times(&mut self){
        self.havoc_outer_times = 1+rand::thread_rng().gen_range(0, config::HAVOC_CYCLES_INIT as u64);
    }

    fn calculate_havoc_inner_times(&mut self){
        self.havoc_inner_times = 1 << (1+rand::thread_rng().gen_range(0, config::HAVOC_STACK_POW2));
    }

    fn state_select_next(&self, len:u64)->FuzzingState {
        if len == 0 {
          return FuzzingState::End;
        }
        FuzzingState::CalHavocTimes
    }

    fn state_cal_havoc_next(&self)->FuzzingState {
      FuzzingState::StateFlip1(0)
      //just for test, we should use the last line
      //FuzzingState::StateInterest8((0,0))
    }

    fn state_flip1_next(&self, len:u64, now_count:u64)->FuzzingState {
        let state_count = (len << 3)-1;
        if now_count < state_count {
          // println!("we are in here {:?}", now_count);
            return FuzzingState::StateFlip1(now_count+1);
        }
        FuzzingState::StateFlip2(0)
    }

    fn state_flip2_next(&self, len:u64, now_count:u64)->FuzzingState {
        let state_count = (len << 3)-2;
        if now_count < state_count {
          // println!("we are in here {:?}", now_count);
            return FuzzingState::StateFlip2(now_count+1);
        }
        FuzzingState::StateFlip4(0)
    }

    fn state_flip4_next(&self, len:u64, now_count:u64)->FuzzingState {
        let state_count = (len << 3)-4;
        if now_count < state_count {
          // println!("we are in here {:?}", now_count);
            return FuzzingState::StateFlip4(now_count+1);
        }
        FuzzingState::StateFlip8(0)
    }

    fn state_flip8_next(&self, len:u64, now_count:u64)->FuzzingState {
        let state_count = len - 1;
        if now_count < state_count {
          // println!("we are in here {:?}", now_count);
            return FuzzingState::StateFlip8(now_count+1);
        }
        //the length of the input is not enough for two bytes
        if len < 2 {
          return FuzzingState::StateAddArith8((0,0));
        }
        FuzzingState::StateFlip16(0)
    }

    fn state_flip16_next(&self, len:u64, now_count:u64)->FuzzingState {
        let state_count = len - 2;
        if now_count < state_count {
            //println!("we are in here {:?}", now_count);
            return FuzzingState::StateFlip16(now_count+1);
        }
        //the length of the input is not enough for four bytes
        if len < 4 {
          return FuzzingState::StateAddArith8((0,0));
        }
        FuzzingState::StateFlip32(0)
    }

    fn state_flip32_next(&self, len:u64, now_count:u64)->FuzzingState {
        let state_count = len - 4;
        if now_count < state_count {
            //println!("we are in here {:?}", now_count);
            return FuzzingState::StateFlip32(now_count+1);
        }
        FuzzingState::StateAddArith8((0,0))
    }

    fn state_arith8_add_next(&self, len:u64, now_count:u64, arith_count:u8)->FuzzingState {
        let state_count = len;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX {
            return FuzzingState::StateAddArith8((now_count, arith_count+1));
          } else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateSubArith8((0,0))
            }
            return FuzzingState::StateAddArith8((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_arith8_sub_next(&self, len:u64, now_count:u64, arith_count:u8)->FuzzingState {
        let state_count = len;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX {
            return FuzzingState::StateSubArith8((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              //the length of the input is not enough for two bytes
              if len < 2 {
                return FuzzingState::StateInterest8((0,0));
              }
              return FuzzingState::StateAddArith16((0, 0));
            }
            return FuzzingState::StateSubArith8((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_arith16_add_next(&self, len:u64, now_count:u64, arith_count:u16)->FuzzingState {
        let state_count = len-1;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX as u16 {
            return FuzzingState::StateAddArith16((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateSubArith16((0, 0));
            }
            return FuzzingState::StateAddArith16((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_arith16_sub_next(&self, len:u64, now_count:u64, arith_count:u16)->FuzzingState {
        let state_count = len-1;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX as u16 {
            return FuzzingState::StateSubArith16((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateAddArith16AnotherEndian((0, 0));
            }
            return FuzzingState::StateSubArith16((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_arith16_add_another_endian_next(&self, len:u64, now_count:u64, arith_count:u16)->FuzzingState {
      let state_count = len-1;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX as u16 {
            return FuzzingState::StateAddArith16AnotherEndian((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateSubArith16AnotherEndian((0, 0));
            }
            return FuzzingState::StateAddArith16AnotherEndian((now_count+1, 0));
          }
            
        }
        FuzzingState::End
    }

    fn state_arith16_sub_another_endian_next(&self, len:u64, now_count:u64, arith_count:u16)->FuzzingState {
      let state_count = len-1;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX as u16 {
            return FuzzingState::StateSubArith16AnotherEndian((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              //the length of the input is not enough for four bytes
              if len < 4 {
                return FuzzingState::StateInterest8((0,0));
              }
              return FuzzingState::StateAddArith32((0,0));
            }
            return FuzzingState::StateSubArith16AnotherEndian((now_count+1, 0));
          }
            
        }
        FuzzingState::End
    }

    fn state_arith32_add_next(&self, len:u64, now_count:u64, arith_count:u32)->FuzzingState {
        let state_count = len-3;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX as u32 {
            return FuzzingState::StateAddArith32((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateSubArith32((0,0));
            }
            return FuzzingState::StateAddArith32((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_arith32_sub_next(&self, len:u64, now_count:u64, arith_count:u32)->FuzzingState {
        let state_count = len-3;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX as u32 {
            return FuzzingState::StateSubArith32((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateAddArith32AnotherEndian((0,0));
            }
            return FuzzingState::StateSubArith32((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_arith32_add_another_endian_next(&self, len:u64, now_count:u64, arith_count:u32)->FuzzingState {
        let state_count = len-3;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX as u32 {
            return FuzzingState::StateAddArith32AnotherEndian((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateSubArith32AnotherEndian((0,0));
            }
            return FuzzingState::StateAddArith32AnotherEndian((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_arith32_sub_another_endian_next(&self, len:u64, now_count:u64, arith_count:u32)->FuzzingState {
        let state_count = len-3;
        if now_count < state_count {
          if arith_count < config::ARITH_MAX as u32 {
            return FuzzingState::StateSubArith32AnotherEndian((now_count, arith_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateInterest8((0,0));
            }
            return FuzzingState::StateSubArith32AnotherEndian((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_interesting8_next(&self, len:u64, now_count:u64, index_count:u8)->FuzzingState {
        let state_count = len;
        if now_count < state_count {
          if index_count < (config::INTERESTING_8_CNT-1) as u8 {
            return FuzzingState::StateInterest8((now_count, index_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              //the length of the input is not enough for two bytes
              if len < 2 {
                return FuzzingState::StateHavoc((0,0));
              }
              return FuzzingState::StateInterest16((0,0));
            }
            return FuzzingState::StateInterest8((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_interesting16_next(&self, len:u64, now_count:u64, index_count:u8)->FuzzingState {
        let state_count = len-1;
        if now_count < state_count {
          if index_count < (config::INTERESTING_16_CNT-1) as u8 {
            return FuzzingState::StateInterest16((now_count, index_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateInterest16AnotherEndian((0,0));
            }
            return FuzzingState::StateInterest16((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_interesting16_another_endian_next(&self, len:u64, now_count:u64, index_count:u8)->FuzzingState {
      let state_count = len-1;
        if now_count < state_count {
          if index_count < (config::INTERESTING_16_CNT-1) as u8 {
            return FuzzingState::StateInterest16AnotherEndian((now_count, index_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              if len < 4 {
                return FuzzingState::StateHavoc((0,0));
              }

              return FuzzingState::StateInterest32((0,0));
            }
            return FuzzingState::StateInterest16AnotherEndian((now_count+1, 0));
          }
            
        }
        FuzzingState::End
    }

    fn state_interesting32_next(&self, len:u64, now_count:u64, index_count:u8)->FuzzingState {
        let state_count = len-3;
        if now_count < state_count {
          if index_count < (config::INTERESTING_32_CNT-1) as u8 {
            return FuzzingState::StateInterest32((now_count, index_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateInterest32AnotherEndian((0,0));
            }
            return FuzzingState::StateInterest32((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_interesting32_another_endian_next(&self, len:u64, now_count:u64, index_count:u8)->FuzzingState {
        let state_count = len-3;
        if now_count < state_count {
          if index_count < (config::INTERESTING_32_CNT-1) as u8 {
            return FuzzingState::StateInterest32AnotherEndian((now_count, index_count+1));
          } 
          else {
            if now_count == state_count-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::StateHavoc((0,0));

              //just for a simple test to skip havoc
              //return FuzzingState::End;
            }
            return FuzzingState::StateInterest32AnotherEndian((now_count+1, 0));
          }
            
        }
        FuzzingState::End    
    }

    fn state_havoc_next(&self, outer_count:u64, inner_count:u8)->FuzzingState {
        if outer_count < self.havoc_outer_times {
          if inner_count < self.havoc_inner_times{
            return FuzzingState::StateHavoc((outer_count, inner_count+1));
          } 
          else {
            if outer_count == self.havoc_outer_times-1 {
              // we have doing all things, let's go to next out State
              return FuzzingState::End;
            }
            return FuzzingState::StateHavoc((outer_count+1, 0));
          }
            
        }
        FuzzingState::End
    }

    pub fn get_next_mutate_state(&mut self, input_seed_len:u64)->FuzzingState {
        match self.mutate_state {
            //最初始状态，更新存储状态，进入选择状态
            FuzzingState::Ready => {
                self.set_seed_len(input_seed_len);
                return FuzzingState::Select
            },
            FuzzingState::Select => {
                return self.state_select_next(self.seed_len)
            },
            FuzzingState::CalHavocTimes => {
                self.calculate_havoc_outer_times();
                self.calculate_havoc_inner_times();
                return self.state_cal_havoc_next()
            },
            FuzzingState::StateFlip1(i) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_flip1_next(self.seed_len, i)
            },
            FuzzingState::StateFlip2(i) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_flip2_next(self.seed_len, i)
            },
            FuzzingState::StateFlip4(i) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_flip4_next(self.seed_len, i)
            },
            FuzzingState::StateFlip8(i) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_flip8_next(self.seed_len, i)
            },
            FuzzingState::StateFlip16(i) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_flip16_next(self.seed_len, i)
            },
            FuzzingState::StateFlip32(i) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_flip32_next(self.seed_len, i)
            },
            FuzzingState::StateAddArith8((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith8_add_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateSubArith8((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith8_sub_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateAddArith16((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith16_add_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateSubArith16((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith16_sub_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateAddArith16AnotherEndian((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith16_add_another_endian_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateSubArith16AnotherEndian((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith16_sub_another_endian_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateAddArith32((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith32_add_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateSubArith32((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith32_sub_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateAddArith32AnotherEndian((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith32_add_another_endian_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateSubArith32AnotherEndian((i,arith_j)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_arith32_sub_another_endian_next(self.seed_len, i, arith_j)
            },
            FuzzingState::StateInterest8((i,index_count)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_interesting8_next(self.seed_len, i, index_count)
            },
            FuzzingState::StateInterest16((i,index_count)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_interesting16_next(self.seed_len, i, index_count)
            },
            FuzzingState::StateInterest16AnotherEndian((i, index_count)) => {
                return self.state_interesting16_another_endian_next(self.seed_len, i, index_count)
            },
            FuzzingState::StateInterest32((i,index_count)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_interesting32_next(self.seed_len, i, index_count)
            },
            FuzzingState::StateInterest32AnotherEndian((i,index_count)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_interesting32_another_endian_next(self.seed_len, i, index_count)
            },
            FuzzingState::StateHavoc((outer_cnt,inner_cnt)) => {
                // println!("we are now in the state {:?}",self.mutate_state);
                return self.state_havoc_next(outer_cnt,inner_cnt)
            },
            _=> return FuzzingState::Ready,
        }
      
    }

    pub fn change_to_next_state(&mut self, next_state: FuzzingState) {
        self.mutate_state = next_state;
        println!("We just finish the state {:?}", self.mutate_state);
        //To do: update next internal state;
    }

    
}