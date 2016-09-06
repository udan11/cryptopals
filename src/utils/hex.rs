/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

// Hex dictionary.
static HEX_DICTIONARY: &'static [u8; 16] = b"0123456789ABCDEF";

pub fn hex_decode(str: &str) -> Vec<u8> {

    assert!(str.len() % 2 == 0);

    // The string is processed as an array of bytes.
    let bytes = str.as_bytes();

    // The length of array is stored for performance reasons.
    let len = bytes.len();

    fn decode(byte: u8) -> u8 {
        match byte {
            b'0'...b'9' => byte - b'0',
            b'A'...b'F' => 10 + byte - b'A',
            b'a'...b'f' => 10 + byte - b'a',
            _ => panic!("invalid hex character"),
        }
    }

    // The result of the decoding process. The decoding process is 4:3.
    let mut ret = vec![0; len / 2];

    // Decoding every group of two characters.
    for i in 0..(len / 2) {
        ret[i] = decode(bytes[2 * i]) << 4 | decode(bytes[2 * i + 1]);
    }

    return ret;
}

pub fn hex_encode(bytes: &Vec<u8>) -> String {

    // The length of array is stored for performance reasons.
    let len = bytes.len();

    // The result of the encoding process. The encoding process is 1:2.
    let mut ret = Vec::with_capacity(2 * len);

    for i in 0..len {
        ret.push(HEX_DICTIONARY[(bytes[i] >> 4) as usize]);
        ret.push(HEX_DICTIONARY[(bytes[i] & 15) as usize]);
    }

    // Returns the encoded text as a UTF-8 string.
    String::from_utf8(ret).unwrap()
}
