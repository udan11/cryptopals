/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

// Decodes a text to a vector of bytes.
pub fn text_decode(str: &str) -> Vec<u8> {
    str.as_bytes().to_vec()
}

// Encodes a vector of bytes as a text.
pub fn text_encode(bytes: &Vec<u8>) -> String {
    String::from_utf8(bytes.clone()).unwrap()
}

// Checks if a text is an ASCII-text.
pub fn is_text(bytes: &Vec<u8>) -> f32 {
    let mut ascii_bytes: u32 = 0u32;
    for i in 0..bytes.len() {
        match bytes[i] {
            // A...Z | a...z | ' '
            65...90 | 97...122 | 32 => ascii_bytes += 3,
            // 0...9
            48...57 => ascii_bytes += 2,
            // Any ASCII character.
            10...127 => ascii_bytes += 1,
            // No other character should be counted in.
            _ => continue,
        }
    }
    ascii_bytes as f32 / (3 * bytes.len()) as f32
}
