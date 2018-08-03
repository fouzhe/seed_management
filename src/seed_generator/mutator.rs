/*
   DeepSAFL - mutate operations
   ------------------------------------------------------

   Written and maintained by Liang Jie <liangjie.mailbox.cn@google.com>
   Copyright 2018. All rights reserved.
*/


#![allow(unused)]



use std;
use std::cmp;
use std::mem;
use super::config;
use rand;
use rand::Rng;


fn flipbit(origin_seed:&mut Vec<u8>, pos:u64) {
    let pos_byte = (pos >> 3) as usize;
    let pos_bit = pos & 7;
    origin_seed[pos_byte] ^= 128 >> pos_bit;
}

pub fn flip_one_bit(input_seed: &Vec<u8>, pos:u64)->Vec<u8> {
    let mut output_seed = input_seed.clone();
    flipbit(&mut output_seed,pos);
    output_seed
}

pub fn flip_one_bit_option(input_seed: &Vec<u8>, pos:u64)->Option<Vec<u8>> {
    let mut output_seed = input_seed.clone();
    flipbit(&mut output_seed,pos);
    Some(output_seed)
}

pub fn flip_two_bits(input_seed: &Vec<u8>, pos:u64)->Vec<u8> {
    let mut output_seed = input_seed.clone();
    flipbit(&mut output_seed,pos);
    flipbit(&mut output_seed,pos+1);
    output_seed
}

pub fn flip_two_bits_option(input_seed: &Vec<u8>, pos:u64)->Option<Vec<u8>>{
    let mut output_seed = input_seed.clone();
    flipbit(&mut output_seed,pos);
    flipbit(&mut output_seed,pos+1);
    Some(output_seed)
}

pub fn flip_four_bits(input_seed: &Vec<u8>, pos:u64)->Vec<u8> {
    let mut output_seed = input_seed.clone();
    flipbit(&mut output_seed,pos);
    flipbit(&mut output_seed,pos+1);
    flipbit(&mut output_seed,pos+2);
    flipbit(&mut output_seed,pos+3);
    output_seed
}

pub fn flip_four_bits_option(input_seed: &Vec<u8>, pos:u64)->Option<Vec<u8>>{
    let mut output_seed = input_seed.clone();
    flipbit(&mut output_seed,pos);
    flipbit(&mut output_seed,pos+1);
    flipbit(&mut output_seed,pos+2);
    flipbit(&mut output_seed,pos+3);
    Some(output_seed)
}

fn flipbyte(origin_seed:&mut Vec<u8>, pos:u64) {
    origin_seed[pos as usize] ^= 0xFF;
}

pub fn flip_one_byte(input_seed:&Vec<u8>, byte_pos:u64)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64);
    let mut output_seed = input_seed.clone();
    flipbyte(&mut output_seed,byte_pos);
    output_seed
}

pub fn flip_one_byte_option(input_seed:&Vec<u8>, byte_pos:u64)->Option<Vec<u8>>{
    assert!(byte_pos < input_seed.len() as u64);
    let mut output_seed = input_seed.clone();
    flipbyte(&mut output_seed,byte_pos);
    Some(output_seed)
}

fn set_one_byte(input_seed:&Vec<u8>, byte_pos:u64, byte_new:u8)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64);
    assert!(byte_new != 0);
    let mut output_seed = input_seed.clone();
    output_seed[byte_pos as usize] = byte_new;
    output_seed
}

pub fn flip_two_bytes(input_seed: &Vec<u8>, byte_pos:u64)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -1);
    let mut output_seed = input_seed.clone();
    flipbyte(&mut output_seed,byte_pos);
    flipbyte(&mut output_seed,byte_pos+1);
    output_seed
}

pub fn flip_two_bytes_option(input_seed: &Vec<u8>, byte_pos:u64)->Option<Vec<u8>> {
    assert!(byte_pos < input_seed.len() as u64 -1);
    let mut output_seed = input_seed.clone();
    flipbyte(&mut output_seed,byte_pos);
    flipbyte(&mut output_seed,byte_pos+1);
    Some(output_seed)
}

pub fn flip_four_bytes(input_seed: &Vec<u8>, byte_pos:u64)->Vec<u8> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    flipbyte(&mut output_seed,byte_pos);
    flipbyte(&mut output_seed,byte_pos+1);
    flipbyte(&mut output_seed,byte_pos+2);
    flipbyte(&mut output_seed,byte_pos+3);
    output_seed
}

pub fn flip_four_bytes_option(input_seed: &Vec<u8>, byte_pos:u64)->Option<Vec<u8>> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    flipbyte(&mut output_seed,byte_pos);
    flipbyte(&mut output_seed,byte_pos+1);
    flipbyte(&mut output_seed,byte_pos+2);
    flipbyte(&mut output_seed,byte_pos+3);
    Some(output_seed)
}

fn could_be_bitflip(xor_val_orign: u32) -> bool {
    let mut sh:u32 = 0;
    let mut xor_val = xor_val_orign;
    if(xor_val == 0) {
        return true;
    }
    while((xor_val & 1) == 0) {
        sh+=1;
        xor_val >>= 1;
    }
    if(xor_val == 1 || xor_val == 3 || xor_val == 15) {
        return true;
    }
    if((sh & 7) != 0) {
        return false;
    }
    if(xor_val == 0xff || xor_val == 0xffff || xor_val == 0xffffffff) {
        return true;
    }
    false
}

// pub fn add_one_byte_could_be_bitflip(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u8)-> bool {
//     let orig = input_seed[byte_pos as usize];
//     let xor_val = orig ^ (orig+arith_number);
//     could_be_bitflip(xor_val as u32)
// }

pub fn arithmetic_add_one_byte(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u8)->Vec<u8> {
    let mut output_seed = input_seed.clone();
    let orig = output_seed[byte_pos as usize];
    output_seed[byte_pos as usize] = orig+arith_number;
    output_seed
}

pub fn arithmetic_add_one_byte_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u8) -> Option<Vec<u8>> {
    let orig = input_seed[byte_pos as usize];
    let xor_val = orig ^ (orig+arith_number);
    if could_be_bitflip(xor_val as u32) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        output_seed[byte_pos as usize] = orig+arith_number;
        Some(output_seed)
    }
}

pub fn sub_one_byte_could_be_bitflip(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u8)-> bool {
    let orig = input_seed[byte_pos as usize];
    let xor_val = orig ^ (orig-arith_number);
    could_be_bitflip(xor_val as u32)
}

pub fn arithmetic_sub_one_byte(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u8)->Vec<u8> {
    let mut output_seed = input_seed.clone();
    let orig = output_seed[byte_pos as usize];
    output_seed[byte_pos as usize] = orig-arith_number;
    output_seed
}

pub fn arithmetic_sub_one_byte_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u8)-> Option<Vec<u8>> {
    let orig = input_seed[byte_pos as usize];
    let xor_val = orig ^ (orig-arith_number);
    if could_be_bitflip(xor_val as u32) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        output_seed[byte_pos as usize] = orig-arith_number;
        Some(output_seed)
    }    
}

// pub fn convert_string_into_two_bytes(pack_data: & String){
//     let ptr :*const u8 = pack_data.as_ptr();
//     let ptr :*const u16 = ptr as *const u16;
//     let s = unsafe{ *ptr};
//     println!("{:?}", s);
// }

// pub fn cast_vec<T:Copy,U:Copy>(v:Vec<T>)->Vec<U>{
//     use std::mem::size_of;
//     use std::mem::forget;
//     let (s_t, s_u) = (size_of::<T>(), size_of::<U>());
//     let bytes_len = v.len() * s_t;
//     assert!( bytes_len % s_u == 0);
//     let mut v = v;
//     v.shrink_to_fit();
//     let len = bytes_len / s_u;
//     let result;
//     unsafe{
//         result = Vec::from_raw_parts(v.as_mut_ptr() as *mut U, len, len);
//         forget(v);
//     }
//     result
// }

//pub fn saturating_add(self, rhs: u16) -> u16
//pub fn saturating_sub(self, rhs: u16) -> u16
//pub fn wrapping_add(self, rhs: u16) -> u16
//pub fn wrapping_sub(self, rhs: u16) -> u16
//pub fn to_bytes(self) -> [u8; 2] let bytes = 0x1234_5678_u32.to_be().to_bytes();
//pub fn from_bytes(bytes: [u8; 2]) -> u16 let int = u32::from_be(u32::from_bytes([0x12, 0x34, 0x56, 0x78]));

// We find repeat codes in following two functions, might be optimized?




// pub fn add_two_bytes_could_be_bitflip(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16)-> bool {

//     assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
//     let first_byte = input_seed[byte_pos as usize] as u16;
//     let second_byte = input_seed[(byte_pos+1) as usize] as u16;
//     let orig_old = first_byte.wrapping_shl(8) + second_byte;
//     let orig_new = orig_old + arith_number;

//     let xor_val = orig_old ^ orig_new;
//     could_be_bitflip(xor_val as u32)
// }

pub fn arithmetic_add_two_bytes(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    let first_byte = output_seed[byte_pos as usize] as u16;
    let second_byte = output_seed[(byte_pos+1) as usize] as u16;


    let orig_old = first_byte.wrapping_shl(8) + second_byte;
    let orig_new = orig_old + arith_number;
    let first_byte_new = (orig_new >> 8) as u8;
    let second_byte_new = (orig_new & 127) as u8;
    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    
    output_seed
}

pub fn arithmetic_add_two_bytes_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16) -> Option<Vec<u8>> {
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let first_byte = input_seed[byte_pos as usize] as u16;
    let second_byte = input_seed[(byte_pos+1) as usize] as u16;
    let orig_old = first_byte.wrapping_shl(8) + second_byte;
    let orig_new = orig_old + arith_number;

    let xor_val = orig_old ^ orig_new;
    if could_be_bitflip(xor_val as u32) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        let first_byte_new = (orig_new >> 8) as u8;
        let second_byte_new = (orig_new & 127) as u8;
        output_seed[byte_pos as usize] = first_byte_new;
        output_seed[(byte_pos+1) as usize] = second_byte_new;
        Some(output_seed)
    }
}

pub fn arithmetic_sub_two_bytes(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    let first_byte = output_seed[byte_pos as usize] as u16;
    let second_byte = output_seed[(byte_pos+1) as usize] as u16;


    let orig_old = first_byte.wrapping_shl(8) + second_byte;
    let orig_new = orig_old - arith_number;
    let first_byte_new = (orig_new >> 8) as u8;
    let second_byte_new = (orig_new & 127) as u8;
    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    
    output_seed
}

pub fn arithmetic_sub_two_bytes_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16)-> Option<Vec<u8>> {
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let first_byte = input_seed[byte_pos as usize] as u16;
    let second_byte = input_seed[(byte_pos+1) as usize] as u16;
    let orig_old = first_byte.wrapping_shl(8) + second_byte;
    let orig_new = orig_old - arith_number;

    let xor_val = orig_old ^ orig_new;
    if could_be_bitflip(xor_val as u32) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        let first_byte_new = (orig_new >> 8) as u8;
        let second_byte_new = (orig_new & 127) as u8;
        output_seed[byte_pos as usize] = first_byte_new;
        output_seed[(byte_pos+1) as usize] = second_byte_new;
        Some(output_seed)
    }
}


pub fn arithmetic_add_two_bytes_another_endian(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    let second_byte = output_seed[byte_pos as usize] as u16;
    let first_byte = output_seed[(byte_pos+1) as usize] as u16;


    let orig_old = first_byte.wrapping_shl(8) + second_byte;
    let orig_new = orig_old + arith_number;
    let second_byte_new = (orig_new >> 8) as u8;
    let first_byte_new = (orig_new & 127) as u8;
    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    
    output_seed
}

pub fn arithmetic_add_two_bytes_another_endian_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16)-> Option<Vec<u8>>{
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let second_byte= input_seed[byte_pos as usize] as u16;
    let first_byte = input_seed[(byte_pos+1) as usize] as u16;
    let orig_old = first_byte.wrapping_shl(8) + second_byte;
    let orig_new = orig_old + arith_number;

    let xor_val = orig_old ^ orig_new;
    if could_be_bitflip(xor_val as u32) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        let second_byte_new = (orig_new >> 8) as u8;
        let first_byte_new = (orig_new & 127) as u8;
        output_seed[byte_pos as usize] = first_byte_new;
        output_seed[(byte_pos+1) as usize] = second_byte_new;
        Some(output_seed)
    }
}

pub fn arithmetic_sub_two_bytes_another_endian(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    let second_byte = output_seed[byte_pos as usize] as u16;
    let first_byte = output_seed[(byte_pos+1) as usize] as u16;


    let orig_old = first_byte.wrapping_shl(8) + second_byte;
    let orig_new = orig_old - arith_number;
    let second_byte_new = (orig_new >> 8) as u8;
    let first_byte_new = (orig_new & 127) as u8;
    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    
    output_seed
}

pub fn arithmetic_sub_two_bytes_another_endian_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u16)-> Option<Vec<u8>>{
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let second_byte= input_seed[byte_pos as usize] as u16;
    let first_byte = input_seed[(byte_pos+1) as usize] as u16;
    let orig_old = first_byte.wrapping_shl(8) + second_byte;
    let orig_new = orig_old - arith_number;

    let xor_val = orig_old ^ orig_new;
    if could_be_bitflip(xor_val as u32) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        let second_byte_new = (orig_new >> 8) as u8;
        let first_byte_new = (orig_new & 127) as u8;
        output_seed[byte_pos as usize] = first_byte_new;
        output_seed[(byte_pos+1) as usize] = second_byte_new;
        Some(output_seed)
    }
}



pub fn arithmetic_add_four_bytes(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u32)->Vec<u8> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    let first_byte = output_seed[byte_pos as usize] as u32;
    let second_byte = output_seed[(byte_pos+1) as usize] as u32;
    let third_byte = output_seed[(byte_pos+2) as usize] as u32;
    let fourth_byte = output_seed[(byte_pos+3) as usize] as u32;


    let orig_old = first_byte.wrapping_shl(24) + second_byte.wrapping_shl(16) +third_byte.wrapping_shl(8) + fourth_byte;
    let orig_new = orig_old + arith_number;
    let first_byte_new = (orig_new >> 24) as u8;
    let second_byte_new = (((orig_new >> 16) & 127).wrapping_shl(24)>>24) as u8;
    let third_byte_new = (((orig_new >> 8) & 127).wrapping_shl(24)>>24) as u8;
    let fourth_byte_new = (orig_new & 127) as u8;

    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    output_seed[(byte_pos+2) as usize] = third_byte_new;
    output_seed[(byte_pos+3) as usize] = fourth_byte_new;
    
    output_seed
}

pub fn arithmetic_add_four_bytes_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u32)-> Option<Vec<u8>> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    
    let first_byte = input_seed[byte_pos as usize] as u32;
    let second_byte = input_seed[(byte_pos+1) as usize] as u32;
    let third_byte = input_seed[(byte_pos+2) as usize] as u32;
    let fourth_byte = input_seed[(byte_pos+3) as usize] as u32;

    let orig_old = first_byte.wrapping_shl(24) + second_byte.wrapping_shl(16) +third_byte.wrapping_shl(8) + fourth_byte;
    let orig_new = orig_old + arith_number;

    let xor_val = orig_old ^ orig_new;
    if could_be_bitflip(xor_val) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        let first_byte_new = (orig_new >> 24) as u8;
        let second_byte_new = (((orig_new >> 16) & 127).wrapping_shl(24)>>24) as u8;
        let third_byte_new = (((orig_new >> 8) & 127).wrapping_shl(24)>>24) as u8;
        let fourth_byte_new = (orig_new & 127) as u8;

        output_seed[byte_pos as usize] = first_byte_new;
        output_seed[(byte_pos+1) as usize] = second_byte_new;
        output_seed[(byte_pos+2) as usize] = third_byte_new;
        output_seed[(byte_pos+3) as usize] = fourth_byte_new;

        Some(output_seed)
    }
}

pub fn arithmetic_sub_four_bytes(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u32)->Vec<u8> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    let first_byte = output_seed[byte_pos as usize] as u32;
    let second_byte = output_seed[(byte_pos+1) as usize] as u32;
    let third_byte = output_seed[(byte_pos+2) as usize] as u32;
    let fourth_byte = output_seed[(byte_pos+3) as usize] as u32;


    let orig_old = first_byte.wrapping_shl(24) + second_byte.wrapping_shl(16) +third_byte.wrapping_shl(8) + fourth_byte;
    let orig_new = orig_old - arith_number;
    let first_byte_new = (orig_new >> 24) as u8;
    let second_byte_new = (((orig_new >> 16) & 127).wrapping_shl(24)>>24) as u8;
    let third_byte_new = (((orig_new >> 8) & 127).wrapping_shl(24)>>24) as u8;
    let fourth_byte_new = (orig_new & 127) as u8;

    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    output_seed[(byte_pos+2) as usize] = third_byte_new;
    output_seed[(byte_pos+3) as usize] = fourth_byte_new;
    
    output_seed
}

pub fn arithmetic_sub_four_bytes_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u32)->Option<Vec<u8>> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    let first_byte = input_seed[byte_pos as usize] as u32;
    let second_byte = input_seed[(byte_pos+1) as usize] as u32;
    let third_byte = input_seed[(byte_pos+2) as usize] as u32;
    let fourth_byte = input_seed[(byte_pos+3) as usize] as u32;

    let orig_old = first_byte.wrapping_shl(24) + second_byte.wrapping_shl(16) +third_byte.wrapping_shl(8) + fourth_byte;
    let orig_new = orig_old - arith_number;

    let xor_val = orig_old ^ orig_new;
    if could_be_bitflip(xor_val) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        let first_byte_new = (orig_new >> 24) as u8;
        let second_byte_new = (((orig_new >> 16) & 127).wrapping_shl(24)>>24) as u8;
        let third_byte_new = (((orig_new >> 8) & 127).wrapping_shl(24)>>24) as u8;
        let fourth_byte_new = (orig_new & 127) as u8;

        output_seed[byte_pos as usize] = first_byte_new;
        output_seed[(byte_pos+1) as usize] = second_byte_new;
        output_seed[(byte_pos+2) as usize] = third_byte_new;
        output_seed[(byte_pos+3) as usize] = fourth_byte_new;

        Some(output_seed)
    }
}

pub fn arithmetic_add_four_bytes_another_endian(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u32)->Vec<u8> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    let fourth_byte = output_seed[byte_pos as usize] as u32;
    let third_byte = output_seed[(byte_pos+1) as usize] as u32;
    let second_byte = output_seed[(byte_pos+2) as usize] as u32;
    let first_byte = output_seed[(byte_pos+3) as usize] as u32;


    let orig_old = first_byte.wrapping_shl(24) + second_byte.wrapping_shl(16) +third_byte.wrapping_shl(8) + fourth_byte;
    let orig_new = orig_old + arith_number;
    let fourth_byte_new = (orig_new >> 24) as u8;
    let third_byte_new = (((orig_new >> 16) & 127).wrapping_shl(24)>>24) as u8;
    let second_byte_new = (((orig_new >> 8) & 127).wrapping_shl(24)>>24) as u8;
    let first_byte_new = (orig_new & 127) as u8;

    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    output_seed[(byte_pos+2) as usize] = third_byte_new;
    output_seed[(byte_pos+3) as usize] = fourth_byte_new;
    
    output_seed
}

pub fn arithmetic_add_four_bytes_another_endian_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u32)->Option<Vec<u8>> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    
    let fourth_byte = input_seed[byte_pos as usize] as u32;
    let third_byte = input_seed[(byte_pos+1) as usize] as u32;
    let second_byte = input_seed[(byte_pos+2) as usize] as u32;
    let first_byte = input_seed[(byte_pos+3) as usize] as u32;


    let orig_old = first_byte.wrapping_shl(24) + second_byte.wrapping_shl(16) +third_byte.wrapping_shl(8) + fourth_byte;
    let orig_new = orig_old + arith_number;

    let xor_val = orig_old ^ orig_new;

    if could_be_bitflip(xor_val) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        let fourth_byte_new = (orig_new >> 24) as u8;
        let third_byte_new = (((orig_new >> 16) & 127).wrapping_shl(24)>>24) as u8;
        let second_byte_new = (((orig_new >> 8) & 127).wrapping_shl(24)>>24) as u8;
        let first_byte_new = (orig_new & 127) as u8;

        output_seed[byte_pos as usize] = first_byte_new;
        output_seed[(byte_pos+1) as usize] = second_byte_new;
        output_seed[(byte_pos+2) as usize] = third_byte_new;
        output_seed[(byte_pos+3) as usize] = fourth_byte_new;

        Some(output_seed)
    }

}

pub fn arithmetic_sub_four_bytes_another_endian(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u32)->Vec<u8> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();
    let fourth_byte = output_seed[byte_pos as usize] as u32;
    let third_byte = output_seed[(byte_pos+1) as usize] as u32;
    let second_byte = output_seed[(byte_pos+2) as usize] as u32;
    let first_byte = output_seed[(byte_pos+3) as usize] as u32;


    let orig_old = first_byte.wrapping_shl(24) + second_byte.wrapping_shl(16) +third_byte.wrapping_shl(8) + fourth_byte;
    let orig_new = orig_old - arith_number;
    let fourth_byte_new = (orig_new >> 24) as u8;
    let third_byte_new = (((orig_new >> 16) & 127).wrapping_shl(24)>>24) as u8;
    let second_byte_new = (((orig_new >> 8) & 127).wrapping_shl(24)>>24) as u8;
    let first_byte_new = (orig_new & 127) as u8;

    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    output_seed[(byte_pos+2) as usize] = third_byte_new;
    output_seed[(byte_pos+3) as usize] = fourth_byte_new;
    
    output_seed
}

pub fn arithmetic_sub_four_bytes_another_endian_option(input_seed: &Vec<u8>, byte_pos:u64, arith_number:u32)->Option<Vec<u8>> {
    assert!(byte_pos < (input_seed.len() as u64) -3); //Attention: You need to ensure the byte_pos is legal
    
    let fourth_byte = input_seed[byte_pos as usize] as u32;
    let third_byte = input_seed[(byte_pos+1) as usize] as u32;
    let second_byte = input_seed[(byte_pos+2) as usize] as u32;
    let first_byte = input_seed[(byte_pos+3) as usize] as u32;


    let orig_old = first_byte.wrapping_shl(24) + second_byte.wrapping_shl(16) +third_byte.wrapping_shl(8) + fourth_byte;
    let orig_new = orig_old - arith_number;

    let xor_val = orig_old ^ orig_new;

    if could_be_bitflip(xor_val) {
        None
    }
    else {
        let mut output_seed = input_seed.clone();
        let fourth_byte_new = (orig_new >> 24) as u8;
        let third_byte_new = (((orig_new >> 16) & 127).wrapping_shl(24)>>24) as u8;
        let second_byte_new = (((orig_new >> 8) & 127).wrapping_shl(24)>>24) as u8;
        let first_byte_new = (orig_new & 127) as u8;

        output_seed[byte_pos as usize] = first_byte_new;
        output_seed[(byte_pos+1) as usize] = second_byte_new;
        output_seed[(byte_pos+2) as usize] = third_byte_new;
        output_seed[(byte_pos+3) as usize] = fourth_byte_new;

        Some(output_seed)
    }
}


// //Helper function to see if a particular value is reachable through
// // arithmetic operations.
// pub fn could_be_arith(old_val: u32, new_val: u32, byte_len: u8) -> bool {
//      let (mut ov, mut nv, mut diffs): (u32, u32, u32) = (0,0,0);
//      if(old_val == new_val) {
//         return 1;
//      }
//      for()

//      false
// }

pub fn interesting8_replace(input_seed: &Vec<u8>, byte_pos:u64, index_number:u8)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();

    assert!(index_number < config::INTERESTING_8_CNT);
    output_seed[byte_pos as usize] = config::INTERESTING_8[index_number as usize] as u8;
    output_seed
}

pub fn interesting16_replace(input_seed: &Vec<u8>, byte_pos:u64, index_number:u8)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();

    assert!(index_number < config::INTERESTING_16_CNT);
    let replace_number = config::INTERESTING_16[index_number as usize];
    let first_byte_new = (replace_number >> 8) as u8;
    let second_byte_new = (replace_number & 127) as u8;

    output_seed[byte_pos as usize] =  first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    output_seed
}

pub fn interesting16_replace_another_endian(input_seed: &Vec<u8>, byte_pos:u64, index_number:u8)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -1); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();

    assert!(index_number < config::INTERESTING_16_CNT);
    let replace_number = config::INTERESTING_16[index_number as usize];
    let second_byte_new = (replace_number >> 8) as u8;
    let first_byte_new = (replace_number & 127) as u8;

    output_seed[byte_pos as usize] =  first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    output_seed
}

pub fn interesting32_replace(input_seed: &Vec<u8>, byte_pos:u64, index_number:u8)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -3); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();

    assert!(index_number < config::INTERESTING_32_CNT); //Attention: index should less than 32_cnt
    let replace_number = config::INTERESTING_32[index_number as usize];

    let first_byte_new = (replace_number >> 24) as u8;
    let second_byte_new = (((replace_number >> 16) & 127).wrapping_shl(24)>>24) as u8;
    let third_byte_new = (((replace_number >> 8) & 127).wrapping_shl(24)>>24) as u8;
    let fourth_byte_new = (replace_number & 127) as u8;

    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    output_seed[(byte_pos+2) as usize] = third_byte_new;
    output_seed[(byte_pos+3) as usize] = fourth_byte_new;

    output_seed
}

pub fn interesting32_replace_another_endian(input_seed: &Vec<u8>, byte_pos:u64, index_number:u8)->Vec<u8> {
    assert!(byte_pos < input_seed.len() as u64 -3); //Attention: You need to ensure the byte_pos is legal
    let mut output_seed = input_seed.clone();

    assert!(index_number < config::INTERESTING_32_CNT); //Attention: index should less than 32_cnt
    let replace_number = config::INTERESTING_32[index_number as usize];

    let fourth_byte_new = (replace_number >> 24) as u8;
    let third_byte_new = (((replace_number >> 16) & 127).wrapping_shl(24)>>24) as u8;
    let second_byte_new = (((replace_number >> 8) & 127).wrapping_shl(24)>>24) as u8;
    let first_byte_new = (replace_number & 127) as u8;

    output_seed[byte_pos as usize] = first_byte_new;
    output_seed[(byte_pos+1) as usize] = second_byte_new;
    output_seed[(byte_pos+2) as usize] = third_byte_new;
    output_seed[(byte_pos+3) as usize] = fourth_byte_new;

    output_seed
}

pub fn delete_byte(input_seed: &Vec<u8>, del_from:u64, del_len:u64)->Vec<u8> {
    assert!(del_from < input_seed.len() as u64);
    assert!(del_len >0);
    assert!(del_from + del_len -1 < input_seed.len() as u64);//del include del_from, del_end is del_from+del_len-1

    let mut output_seed = input_seed.clone();
    output_seed.drain((del_from as usize)..((del_from+del_len) as usize));

    output_seed

}

//This function in AFL needs other information, 
//namely, 1.queue_cycle(cycles for passing the queue) 2.run_over10m(whether the fuzzer running for 10 minutes)
//Here we first do not use these information, may be need update in future ... Why need?

// Helper to choose random block len for block operations in fuzz_one().
//    Doesn't return zero, provided that max_len is > 0.

pub fn choose_block_len(limit:u64, rang:& mut rand::ThreadRng) -> u64 {
    let mut min_value:u64;
    let mut max_value:u64;
    let rlim = 3; //afl use MIN(queue_cycle, 3), here we simplify it to directly use 3
    match rang.gen_range(0, rlim) {
        0 => {
            min_value = 1;
            max_value = config::HAVOC_BLK_SMALL;
        },
        1 => {
            min_value = config::HAVOC_BLK_SMALL;
            max_value = config::HAVOC_BLK_MEDIUM;
        }
        _ => {
            if rang.gen_range(0,10) != 0 {
                min_value = config::HAVOC_BLK_MEDIUM;
                max_value = config::HAVOC_BLK_LARGE;
            }
            else {
                min_value = config::HAVOC_BLK_LARGE;
                max_value = config::HAVOC_BLK_XL;
            }
        }
    }
    if min_value >= limit {
        min_value = 1;
    }
    let interval = rang.gen_range(0, cmp::min(max_value, limit)-min_value+1);
    min_value + interval

}

pub fn insert_clone_bytes(input_seed: &Vec<u8>, rang:& mut rand::ThreadRng)->Vec<u8> {
    //We clone the input_seed from the clone_start_pos to clone_start_pos+clone_len-1
    //We insert the clone bytes to the insert_pos
    assert!(input_seed.len() as u64 + config::HAVOC_BLK_XL < config::MAX_FILE);//how to 
    let (clone_start_pos, clone_len, insert_pos):(usize, usize, usize);
    let len = input_seed.len() as u64;
    let mut output_seed = input_seed.clone();

    //If is_clone_from_old_string does not equal to 0, we clone bytes from old string
    //else, we random set a block of bytes. 
    let is_clone_from_old_string = rang.gen_range(0, 4);


    if(is_clone_from_old_string == 0) {
        clone_len = choose_block_len(config::HAVOC_BLK_XL, rang) as usize;
        clone_start_pos = 0;
    }
    else {
        clone_len = choose_block_len(len,rang) as usize;
        assert!(0 < len-(clone_len as u64)+1);
        clone_start_pos = rang.gen_range(0, len-(clone_len as u64)+1) as usize;
    }
    let insert = rang.gen_range(0, len) as usize;
    let mut temp_seed = input_seed.clone();
    let insert_block: Vec<_>;

    if(is_clone_from_old_string == 0) {
        if(rang.gen_range(0,2) == 0) {
            let pad = rang.gen_range(0,256 as u16) as u8;
            insert_block = vec![pad;clone_len];
        }
        else {
            let pad_pos = rang.gen_range(0,len) as usize;
            let pad = output_seed[pad_pos];
            insert_block = vec![pad;clone_len];
        }
        
    }
    else {
        insert_block = temp_seed.drain(clone_start_pos..clone_start_pos+clone_len).collect();
    }
     
    output_seed.splice(insert..insert, insert_block.iter().cloned());

    output_seed
}

// pub fn splice(input_seed: &Vec<u8>, random_input_seed: &Vec<u8>, rang:& mut rand::ThreadRng)->Vec<u8> {
//     assert!(output_seed.len() < 2);
//     output_seed = input_seed.clone();
    
    
//     output_seed
// }

pub fn havoc_mutate(input_seed: &Vec<u8>, rang:& mut rand::ThreadRng)->Vec<u8> {
    // let mut random_value = rang.gen_range(0,config::HAVOC_WAY as u64);
    let len = input_seed.len() as u64;

    let max_random_value = match len{
                            1 => {
                                9
                            }
                            2...3 => {
                                15
                            }
                            _ => config::HAVOC_WAY as u32
                           };

    let random_value = rang.gen_range(0,max_random_value);

    match random_value {
        //0--8 operations need one byte at least 
        0 => {
            //println!("we are using flipping bit");
            flip_one_bit(input_seed, rang.gen_range(0,len.wrapping_shl(3)))
        },
        1 => {
            // println!("we are using flipping two bits");
            let pos = rang.gen_range(0, len.wrapping_shl(3)-1);
            flip_two_bits(input_seed, pos)  
        },
        2 => {
            // println!("we are using flipping four bits");
            let pos = rang.gen_range(0, len.wrapping_shl(3)-3);
            flip_four_bits(input_seed, pos)  
        },
        3 => {
            // println!("we are using flipping one byte");
            let pos = rang.gen_range(0, len);
            flip_one_byte(input_seed, pos)
        },
        4 => {
            // println!("we are using arith_add_one_byte");
            let pos = rang.gen_range(0, len);
            let arith_number = rang.gen_range(0, config::ARITH_MAX);
            arithmetic_add_one_byte(input_seed, pos, arith_number)
        },
        5 => {
            // println!("we are using arith_sub_one_byte");
            let pos = rang.gen_range(0, len);
            let arith_number = rang.gen_range(0, config::ARITH_MAX);
            arithmetic_sub_one_byte(input_seed, pos, arith_number)
        },
        6 => {
            // println!("we are using interesting8");
            let pos = rang.gen_range(0,len);
            let index_number = rang.gen_range(0,config::INTERESTING_8_CNT);
            interesting8_replace(input_seed, pos, index_number)
        },
        7 => {
            // println!("We are setting a random byte with a random value");
            let pos = rang.gen_range(0, len);
            let random_byte_value = 1 + rang.gen_range(0,255);
            set_one_byte(input_seed, pos, random_byte_value)
        },
        8 => {
            //afl-fuzz 13 Extra-large blocks, selected very rarely (<5% of the time)
            if input_seed.len() as u64 + config::HAVOC_BLK_XL < config::MAX_FILE {
                // println!("We are inserting the clone bytes");    
                insert_clone_bytes(input_seed, rang)
            }
            else {
                //not a good solution when the length is too long, but we first deal with it in this way
                //need a more elegant way in the second edition (like return a Result and deal with Err in lib.rs)
                let pos = rang.gen_range(0, len);
                let random_byte_value = 1 + rang.gen_range(0,255);
                set_one_byte(input_seed, pos, random_byte_value)
            }
            
        },
        //9--14 operations need two bytes at least
        9 => {
            // println!("we are using flipping two bytes");
            let pos = rang.gen_range(0, len-1);
            flip_two_bytes(input_seed, pos)
        },
        10 => {
            // println!("we are using arithmetic_add_two_bytes, randomly choose endian");
            let pos = rang.gen_range(0, len-1);            
            let arith_number = rang.gen_range(0, config::ARITH_MAX);
            let use_another = rang.gen_range(0,2);
            if use_another == 1 {
                arithmetic_add_two_bytes_another_endian(input_seed, pos, arith_number as u16)
            }
            else {
                arithmetic_add_two_bytes(input_seed, pos, arith_number as u16)
            }
        },
        11 => {
            // println!("we are using arithmetic_sub_two_bytes, randomly choose endian");
            let pos = rang.gen_range(0, len-1);            
            let arith_number = rang.gen_range(0, config::ARITH_MAX);
            let use_another = rang.gen_range(0,2);
            if use_another == 1 {
                arithmetic_sub_two_bytes_another_endian(input_seed, pos, arith_number as u16)
            }
            else {
                arithmetic_sub_two_bytes(input_seed, pos, arith_number as u16)
            }
        },
        12 => {
            // println!("we are using interesting16, randomly choose endian");
            let pos = rang.gen_range(0, len-1);            
            let index_number = rang.gen_range(0,config::INTERESTING_16_CNT);
            let use_another = rang.gen_range(0,2);
            if use_another == 1 {
                interesting16_replace(input_seed, pos, index_number)
            }
            else {
                interesting16_replace_another_endian(input_seed, pos, index_number)
            }
            
        },
        13 ...14 => {
            //need least two bytes
            // println!("we try to delete bytes");
            let mut del_from:u64;
            let mut del_len:u64;
            
            del_len = choose_block_len(len-1, rang);
            del_from = rang.gen_range(0, len - del_len + 1);
            delete_byte(input_seed, del_from, del_len)
        },
        //15--18 operations need four bytes at least
        15 => {
            // println!("we are using flipping four bytes");
            let pos = rang.gen_range(0, len-3);
            flip_four_bytes(input_seed, pos)
        },
        16 => {
            // println!("we are using arithmetic_add_four_bytes, randomly choose endian");
            let pos = rang.gen_range(0, len-3);
            let arith_number = rang.gen_range(0, config::ARITH_MAX);
            let use_another = rang.gen_range(0,2);
            if use_another == 1 {
                arithmetic_add_four_bytes_another_endian(input_seed, pos, arith_number as u32)
            }
            else {
                arithmetic_add_four_bytes(input_seed, pos, arith_number as u32)
            }
        },
        17 => {
            // println!("we are using arithmetic_sub_four_bytes, randomly choose endian");
            let pos = rang.gen_range(0, len-3);
            let arith_number = rang.gen_range(0, config::ARITH_MAX);
            let use_another = rang.gen_range(0,2);
            if use_another == 1 {
                arithmetic_sub_four_bytes_another_endian(input_seed, pos, arith_number as u32)
            }
            else {
                arithmetic_sub_four_bytes(input_seed, pos, arith_number as u32)
            }
        },
        18 => {
            // println!("we are using interesting32, randomly choose endian");
            let pos = rang.gen_range(0, len-3);
            let index_number = rang.gen_range(0,config::INTERESTING_32_CNT);
            let use_another = rang.gen_range(0,2);
            if use_another == 1 {
                interesting32_replace(input_seed, pos, index_number)
            }
            else {
                interesting32_replace_another_endian(input_seed, pos, index_number)
            }
        },
        
        
        19 => {
            //afl-case 14
            input_seed.clone()
        },
        _ => input_seed.clone()
    }
}