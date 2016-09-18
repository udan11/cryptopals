/**
 * Copyright (c) 2016, Dan Ungureanu
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided the author has given his consent.
 */

extern crate crypto;

pub mod utils;
pub mod s01;
pub mod s02;

static CHALLENGES: [fn(); 10] = [s01::c01, s01::c02, s01::c03, s01::c04, s01::c05, s01::c06,
                                s01::c07, s01::c08, s02::c09, s02::c10];

fn run_challenge(i: usize) {
    println!("================================================================================");
    println!("            Challenge # {}", i);
    println!("--------------------------------------------------------------------------------");
    CHALLENGES[i - 1]();
}

fn main() {
    // Checking if any specific challenge should be executed, otherwise runs all
    // challenges.
    // e.g. `cargo run 1 2 4` runs only challenges 1, 2 and 4
    // e.g. `cargo run`       runs all challenges
    if std::env::args().len() > 1 {
        for argument in std::env::args().skip(1) {
            let i = argument.parse::<usize>().unwrap();
            run_challenge(i);
        }
    } else {
        for i in 0..CHALLENGES.len() {
            run_challenge(i + 1);
        }
    }
}
