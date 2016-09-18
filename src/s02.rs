/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

use crypto::aessafe;
use crypto::symmetriccipher::BlockDecryptor;
use utils::*;

pub fn aes128_cbc_decrypt(ciphertext: &Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Vec<u8> {
    let size = iv.len();
    let mut xor_key = iv.clone();
    let mut res = Vec::new();
    for block in ciphertext.chunks(size) {
        let mut decrypted = [0; 16];
        let decryptor = aessafe::AesSafe128Decryptor::new(key);
        decryptor.decrypt_block(block, &mut decrypted);
        res.extend(xor_encrypt(&decrypted.to_vec(), &xor_key).iter());
        xor_key = block.to_vec();
    }
    res
}

pub fn c09() {
    const SIZE: usize = 20;
    let text = b"YELLOW SUBMARINE";
    println!("{}", text_encode(&pcks7_pad(&text.to_vec(), SIZE)));
}

pub fn c10() {
    let ciphertext = base64_decode(&get_bytes("res/10.txt"));
    let key = b"YELLOW SUBMARINE";
    let text = aes128_cbc_decrypt(&ciphertext, &key.to_vec(), &[0; 16].to_vec());
    println!("{}", text_encode(&pcks7_unpad(&text)));
}
