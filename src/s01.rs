/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

use crypto;
use crypto::{aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, RefReadBuffer, RefWriteBuffer};
use std::collections::HashSet;
use utils::*;

pub fn c01() {
    let text = "49276d206b696c6c696e6720796f757220627261696e206c\
                       696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", base64_encode(&hex_decode(&text)));
}

pub fn c02() {
    let text = "1c0111001f010100061a024b53535009181c";
    let key = "686974207468652062756c6c277320657965";

    println!("{}",
             hex_encode(&xor_encrypt(&hex_decode(&text), &hex_decode(&key))));
}

pub fn c03() {
    let ciphertext = hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let key = xor_decrypt_single(&ciphertext);
    println!("key = {key} => {text}",
             key = key,
             text = text_encode(&xor_encrypt_single(&ciphertext, key)));
}

pub fn c04() {
    let mut best_index = 0;
    let mut best_key = 0;
    let mut best_text = Vec::new();
    let mut best_ratio = 0f32;

    for (index, line) in get_lines("s01c04.txt").enumerate() {
        let buff = line.unwrap();
        let key = xor_decrypt_single(&hex_decode(&buff));
        let text = xor_encrypt_single(&hex_decode(&buff), key);
        let ratio = is_text(&text);
        if best_ratio < ratio {
            best_index = index;
            best_key = key;
            best_text = text;
            best_ratio = ratio;
        }
    }

    println!("index = {index}, key = {key} => {text}",
             index = best_index,
             key = best_key,
             text = text_encode(&best_text));
}

pub fn c05() {
    let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";

    println!("{}",
             hex_encode(&xor_encrypt(&text_decode(&text), &text_decode(&key))));
}

pub fn c06() {
    let ciphertext = base64_decode(&get_bytes("s01c06.txt"));
    let key = xor_decrypt(&ciphertext);
    println!("{}", text_encode(&xor_encrypt(&ciphertext, &key)));
}

pub fn c07() {
    let key = b"YELLOW SUBMARINE";
    let ciphertext = base64_decode(&get_bytes("s01c07.txt"));

    let mut inbuf = RefReadBuffer::new(&ciphertext);
    let mut outbuf_storage = [0; 8192];
    let mut outbuf = RefWriteBuffer::new(&mut outbuf_storage);
    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128, key, blockmodes::PkcsPadding);
    let mut decrypted = Vec::new();

    loop {
        let result = decryptor.decrypt(&mut inbuf, &mut outbuf, true).unwrap();
        decrypted.extend(outbuf.take_read_buffer().take_remaining());
        if let crypto::buffer::BufferResult::BufferUnderflow = result {
            break;
        }
    }

    println!("{}", text_encode(&decrypted));
}

pub fn c08() {
    let size = 16;
    for (index, line) in get_lines("s01c08.txt").enumerate() {
        let ciphertext = hex_decode(&line.unwrap());
        let mut set = HashSet::new();
        for chunk in ciphertext.chunks(size) {
            if !set.insert(chunk) {
                println!("line {} has been ciphertext with ECB", index);
                break;
            }
        }
    }
}
