/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

use utils::*;

#[test]
fn test_text_decode() {
    // TODO.
}

#[test]
fn test_text_encode() {
    // TODO.
}

#[test]
fn test_is_text() {
    // TODO.
}

#[test]
fn test_hex_encode() {
    // TODO.
}

#[test]
fn test_hex_decode() {
    // TODO.
}

#[test]
fn test_base64_encode() {
    assert!(base64_encode(&"pleasure.".as_bytes().to_vec()) == "cGxlYXN1cmUu");
    assert!(base64_encode(&"leasure.".as_bytes().to_vec()) == "bGVhc3VyZS4=");
    assert!(base64_encode(&"easure.".as_bytes().to_vec()) == "ZWFzdXJlLg==");
    assert!(base64_encode(&"asure.".as_bytes().to_vec()) == "YXN1cmUu");
    assert!(base64_encode(&"sure.".as_bytes().to_vec()) == "c3VyZS4=");
}

#[test]
fn test_base64_decode() {
    assert!(String::from_utf8(base64_decode("cGxlYXN1cmUu")).unwrap() == "pleasure.");
    assert!(String::from_utf8(base64_decode("bGVhc3VyZS4=")).unwrap() == "leasure.");
    assert!(String::from_utf8(base64_decode("ZWFzdXJlLg==")).unwrap() == "easure.");
    assert!(String::from_utf8(base64_decode("YXN1cmUu")).unwrap() == "asure.");
    assert!(String::from_utf8(base64_decode("c3VyZS4=")).unwrap() == "sure.");
}

#[test]
fn test_xor_encrypt() {
    // TODO.
}

#[test]
fn test_xor_encrypt_single() {
    // TODO.
}

#[test]
fn test_hamming_distance() {
    let a = text_decode("this is a test");
    let b = text_decode("wokka wokka!!!");
    assert!(hamming_distance(&a, &b) == 37);
}
