/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::fs::File;

pub use self::base64::*;
pub use self::hex::*;
pub use self::text::*;
pub use self::xor::*;

mod base64;
mod hex;
mod text;
mod xor;

pub fn get_lines(filename: &str) -> Lines<BufReader<File>> {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines()
}

pub fn get_bytes(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut content = String::new();
    for (_, line) in reader.lines().enumerate() {
        content.push_str(line.unwrap().trim());
    }
    content
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    if a.len() != b.len() {
        return 0;
    }

    let mut ret: u32 = 0;
    for i in 0..a.len() {
        for j in 0..8 {
            ret += if ((a[i] ^ b[i]) & (1 << j)) != 0 {
                1
            } else {
                0
            }
        }
    }

    ret
}
