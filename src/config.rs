pub const SKIP_TO_NEW_PROB: u32 = 99;
pub const ARITH_MAX:u8 = 35;
pub const INTERESTING_8_CNT:u8 = 9;
pub const INTERESTING_8: &[i8] = &[
      -128,          // Overflow signed 8-bit when decremented 
      -1,            //                                         
       0,            //                                         
       1,            //                                         
       16,           // One-off with common buffer size         
       32,           // One-off with common buffer size         
       64,           // One-off with common buffer size         
       100,          // One-off with common buffer size         
       127           // Overflow signed 8-bit when incremented     
];

pub const INTERESTING_16_CNT:u8 = 19;
pub const INTERESTING_16: &[i16] = &[
    -128,          // Overflow signed 8-bit when decremented 
    -1,            //                                         
     0,            //                                         
     1,            //                                         
     16,           // One-off with common buffer size         
     32,           // One-off with common buffer size         
     64,           // One-off with common buffer size         
     100,          // One-off with common buffer size         
     127,          // Overflow signed 8-bit when incremented 
    -32768,        // Overflow signed 16-bit when decremented  
    -129,          // Overflow signed 8-bit                    
     128,          // Overflow signed 8-bit                    
     255,          // Overflow unsig 8-bit when incremented    
     256,          // Overflow unsig 8-bit                     
     512,          // One-off with common buffer size          
     1000,         // One-off with common buffer size          
     1024,         // One-off with common buffer size          
     4096,         // One-off with common buffer size          
     32767         // Overflow signed 16-bit when incremented   
];

pub const INTERESTING_32_CNT:u8 = 27;
pub const INTERESTING_32: &[i32] = &[
    -128,          // Overflow signed 8-bit when decremented 
    -1,            //                                         
     0,            //                                         
     1,            //                                         
     16,           // One-off with common buffer size         
     32,           // One-off with common buffer size         
     64,           // One-off with common buffer size         
     100,          // One-off with common buffer size         
     127,          // Overflow signed 8-bit when incremented 
    -32768,        // Overflow signed 16-bit when decremented  
    -129,          // Overflow signed 8-bit                    
     128,          // Overflow signed 8-bit                    
     255,          // Overflow unsig 8-bit when incremented    
     256,          // Overflow unsig 8-bit                     
     512,          // One-off with common buffer size          
     1000,         // One-off with common buffer size          
     1024,         // One-off with common buffer size          
     4096,         // One-off with common buffer size          
     32767,        // Overflow signed 16-bit when incremented
    -2147483648,   // Overflow signed 32-bit when decremented
    -100663046,    // Large negative number (endian-agnostic) 
    -32769,        // Overflow signed 16-bit                  
     32768,        // Overflow signed 16-bit                   
     65535,        // Overflow unsig 16-bit when incremented   
     65536,        // Overflow unsig 16 bit                    
     100663045,    // Large positive number (endian-agnostic)  
     2147483647    // Overflow signed 32-bit when incremented
];
 

pub const HAVOC_MIN:u8 = 16;

//Baseline number of random tweaks during a single 'havoc' stage
pub const HAVOC_CYCLES:u16 = 256;
pub const HAVOC_CYCLES_INIT:u16 = 1024;

pub const SPLICE_HAVOC:u8 = 32;

// Maximum stacking for havoc-stage tweaks. The actual value is calculated
//    like this: 

//    n = random between 1 and HAVOC_STACK_POW2
//    stacking = 2^n

//    In other words, the default (n = 7) produces 2, 4, 8, 16, 32, 64, or
//    128 stacked tweaks:

pub const HAVOC_STACK_POW2:u8 = 7;
pub const HAVOC_WAY:u8 = 19;

// Caps on block sizes for cloning and deletion operations. Each of these
// ranges has a 33% probability of getting picked, except for the first
// two cycles where smaller blocks are favored:

pub const HAVOC_BLK_SMALL:u64 = 32;
pub const HAVOC_BLK_MEDIUM: u64 = 128;
pub const HAVOC_BLK_LARGE: u64 = 1500;
pub const HAVOC_BLK_XL:u64 = 32768; // Extra-large blocks, selected very rarely (<5% of the time)

pub const MAX_FILE:u64 = 1*1024*1024; //Maximum size of input file, in bytes (keep under 100MB)
pub const MAP_SIZE:usize = 1 << 16;
