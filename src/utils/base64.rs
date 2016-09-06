/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

// Base64 dictionary.
static BASE64_DICTIONARY: &'static [u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Decodes a string of characters into a string of bytes.
pub fn base64_decode(str: &str) -> Vec<u8> {

    // Every 3-bytes from the original byte string are converted into 4
    // characters, which means the input must be a multiple of 4.
    assert!(str.len() % 4 == 0);

    // Decodes a 8-bit base64 character into a 6-bit character.
    fn decode(byte: u8) -> u8 {
        match byte {
            b'A'...b'Z' => byte - b'A',
            b'a'...b'z' => 26 + byte - b'a',
            b'0'...b'9' => 52 + byte - b'0',
            b'+' => 62,
            b'/' => 63,
            _ => panic!("invalid base64 character"),
        }
    }

    // The string is processed as an array of bytes.
    let bytes = str.as_bytes();

    // The length of array is stored for performance reasons.
    let len = bytes.len();

    // The number of padding bytes is determined by seeing how many padding
    // characters were added at the end of the string.
    // There may be at most two equals characters.
    let padding_bytes = (bytes[len - 2] == b'=') as usize + (bytes[len - 1] == b'=') as usize;

    // The number of complete chunks (that do not contain any padding bytes).
    let chunks_no = (len - padding_bytes) / 4;

    // The result of the decoding process. The decoding process is 4:3.
    let mut ret = vec![0; 3 * len / 4 - padding_bytes];

    // Decoding the first `chunks_no` chunks of 4 bytes.
    // All of these chunks are complete with no padding bytes.
    for i in 0..chunks_no {
        let a = decode(bytes[4 * i]);
        let b = decode(bytes[4 * i + 1]);
        let c = decode(bytes[4 * i + 2]);
        let d = decode(bytes[4 * i + 3]);
        ret[3 * i] = a << 2 | b >> 4;     // aaaaaabb
        ret[3 * i + 1] = b << 4 | c >> 2; // bbbbcccc
        ret[3 * i + 2] = c << 6 | d;      // ccdddddd
    }

    // The entire decoding is processed in the for above if there are no padding
    // bytes. Otherwise, the last chunk is decoded below.
    if padding_bytes == 0 {
        return ret;
    }

    // Decoding the last chunk.
    let a = decode(bytes[4 * chunks_no]);
    let b = decode(bytes[4 * chunks_no + 1]);
    if padding_bytes == 1 {
        let c = decode(bytes[4 * chunks_no + 2]);
        ret[3 * chunks_no] = a << 2 | b >> 4; // aaaaaabb
        ret[3 * chunks_no + 1] = b << 4 | c >> 2; // bbbbcccc
    } else if padding_bytes == 2 {
        ret[3 * chunks_no] = a << 2 | b >> 4; // aaaaaabb
    }

    return ret;
}

pub fn base64_encode(bytes: &Vec<u8>) -> String {

    // The length of array is stored for performance reasons.
    let len = bytes.len();

    // The number of complete chunks (that do not require any padding bytes).
    let chunks_no = len / 3;

    // The result of the encoding process. The encoding process is 3:4.
    let mut ret = Vec::with_capacity(4 * chunks_no);

    // Encoding the first `chunks_no` chunks of 3 bytes.
    // All of these chunks are complete and require no padding bytes.
    for i in 0..chunks_no {
        ret.push(BASE64_DICTIONARY[(0x3F & (bytes[3 * i] >> 2)) as usize]);
        ret.push(BASE64_DICTIONARY[(0x3F & (bytes[3 * i] << 4 | bytes[3 * i + 1] >> 4)) as usize]);
        ret.push(BASE64_DICTIONARY[(0x3F &
                                    (bytes[3 * i + 1] << 2 |
                                     bytes[3 * i + 2] >>
                                     6)) as usize]);
        ret.push(BASE64_DICTIONARY[(0x3F & bytes[3 * i + 2]) as usize]);
    }

    // Encoding the last chunk and adding an appropriate number of padding
    // bytes.
    let remaining_bytes = len % 3;
    if remaining_bytes == 1 {
        ret.push(BASE64_DICTIONARY[(0x3F & (bytes[3 * chunks_no] >> 2)) as usize]);
        ret.push(BASE64_DICTIONARY[(0x3F & (bytes[3 * chunks_no] << 4)) as usize]);
        ret.push(b'=');
        ret.push(b'=');
    } else if remaining_bytes == 2 {
        ret.push(BASE64_DICTIONARY[(0x3F & (bytes[3 * chunks_no] >> 2)) as usize]);
        ret.push(BASE64_DICTIONARY[(0x3F &
                                    (bytes[3 * chunks_no] << 4 |
                                     bytes[3 * chunks_no + 1] >>
                                     4)) as usize]);
        ret.push(BASE64_DICTIONARY[(0x3F & (bytes[3 * chunks_no + 1] << 2)) as usize]);
        ret.push(b'=');
    }

    // Returns the encoded text as a UTF-8 string.
    String::from_utf8(ret).unwrap()
}
