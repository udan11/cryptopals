/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

use utils::*;

pub fn xor_encrypt(msg: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut ret = Vec::with_capacity(msg.len());
    for i in 0..msg.len() {
        ret.push(msg[i] ^ key[i % key.len()]);
    }
    ret
}

pub fn xor_encrypt_single(msg: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut ret = Vec::with_capacity(msg.len());
    for i in 0..msg.len() {
        ret.push(msg[i] ^ key);
    }
    ret
}

pub fn xor_decrypt(msg: &Vec<u8>) -> Vec<u8> {
    // Finding the size of the key.
    let mut min_ratio = ::std::f32::INFINITY;
    let mut keysize = 0;
    for size in 2..40 {
        let chunks = msg.chunks(size);
        let mut sum = 0;
        for i in 0..chunks.len() - 1 {
            let fst = msg.chunks(size).nth(i).unwrap();
            let snd = msg.chunks(size).nth(i + 1).unwrap();
            sum += hamming_distance(fst, snd);
        }
        let ratio = sum as f32 / (chunks.len() * size) as f32;
        if ratio < min_ratio {
            min_ratio = ratio;
            keysize = size;
        }
    }
    println!("keysize = {}", keysize);

    // Trying to find the key.
    let mut key = Vec::new();
    for i in 0..keysize {
        let mut block = Vec::new();
        for j in i..msg.len() {
            if j % keysize != i {
                continue;
            }
            block.push(msg[j]);
        }
        key.push(xor_decrypt_single(&block));
    }

    key
}

pub fn xor_decrypt_single(msg: &Vec<u8>) -> u8 {
    let mut best_ratio = 0f32;
    let mut best_key = 0;
    for key in 0..255 {
        let ratio = is_text(&xor_encrypt_single(msg, key));
        if best_ratio < ratio {
            best_ratio = ratio;
            best_key = key;
        }
    }
    best_key
}
