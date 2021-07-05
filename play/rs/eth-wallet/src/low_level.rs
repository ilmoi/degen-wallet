use rand::{thread_rng, Rng};
use std::cmp;
use std::iter::FromIterator;
use std::mem::transmute;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    // 1 Create a cryptographically random sequence S of 128 to 256 bits.
    // docs say thread_rng is cryptographically secure - https://docs.rs/rand/0.5.0-pre.0/rand/fn.thread_rng.html
    let mut rng = thread_rng();
    let entropy: u128 = rng.gen();
    println!("Entropy is: {}", entropy);

    // 2 Create a checksum of S by taking the first length-of-S ÷ 32 bits of the SHA-256 hash of S.
    // assume big endian machine
    // convert to bytes first as per this thread - https://users.rust-lang.org/t/how-to-serialize-a-u32-into-byte-array/986
    let bytes: [u8; 16] = unsafe { transmute(entropy.to_be()) };
    // apply sha256
    let hash = hmac_sha256::Hash::hash(&bytes);
    // println!("hash is {:?}", hash);
    // println!("hash[0] is {:b}", hash[0]);
    // take the first 128/32 = 4 bits
    let four_bits = hash[0] >> 4;
    println!("Four checksum bits are: {:b}", four_bits);

    // 3 Add the checksum to the end of the random sequence S.
    //pad with 0s
    let four_bits_str = format!("{:b}", four_bits);
    let four_bits_str = format!("{}{}", "0".repeat(4 - four_bits_str.len()), four_bits_str);
    let entropy_str = format!("{:b}", entropy);
    let entropy_str = format!("{}{}", "0".repeat(128 - entropy_str.len()), entropy_str);
    // prep a single long string of 1s and 0s, total 132 in length
    let checksumed_bytes = format!("{}{}", entropy_str, four_bits_str);
    println!(
        "Full checksummed string of bits is: {}",
        checksumed_bytes.len()
    );

    // 4 Divide the sequence-and-checksum concatenation into sections of 11 bits.
    let sections = split_string(&checksumed_bytes, 11);
    let sections: Vec<u16> = sections.iter().map(|s| convert(s)).collect();
    println!("11-bit sections (as 1-2048 ints) are: {:?}", sections);

    // 5 Map each 11-bit value to a word from the predefined dictionary of 2,048 words.
    let words_dict = lines_from_file("./src/word_dict.txt");

    // 6 Create the mnemonic code from the sequence of words, maintaining the order.
    let mnemonic: Vec<String> = sections
        .iter()
        .map(|s| format!("{} ", words_dict[*s as usize].clone()))
        .collect();
    let mnemonic_str = String::from_iter(mnemonic);
    println!("Mnemonic is: {}", mnemonic_str);

    // -----------------------------------------------------------------------------
    // let's be real, the above is most likely wrong:) I'm fuck all cryptographer. But it was fun to go through the exercise.
    // not gonna bother with the rest.
    // let's use a lib put together by people who know what the fuck they're doing - https://docs.rs/tiny-bip39/0.8.0/bip39/

    // 7 The first parameter to the PBKDF2 key-stretching function is the mnemonic produced in step 6.

    // 8 The second parameter to the PBKDF2 key-stretching function is a salt. The salt is composed of the string constant "mnemonic" concatenated with an optional user-supplied passphrase.

    // 9 PBKDF2 stretches the mnemonic and salt parameters using 2,048 rounds of hashing with the HMAC-SHA512 al/gorithm, producing a 512-bit value as its final output. That 512-bit value is the seed.
}

fn convert(bits: &str) -> u16 {
    let mut result: u16 = 0;
    for c in bits.chars() {
        let bit = c.to_digit(10).unwrap() as u16;
        // println!("bit is {}", bit);
        result <<= 1;
        result ^= bit;
    }
    result
}

fn split_string(string: &str, sub_len: usize) -> Vec<&str> {
    let mut v = vec![];
    let mut cur = string;
    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(cmp::min(sub_len, cur.len()));
        v.push(chunk);
        cur = rest;
    }
    v
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
