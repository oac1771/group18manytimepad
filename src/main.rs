use std::fs::File;
use std::io::prelude::*;

use num::BigUint;

fn main() {
    // Retrieves the conversation from a file
    let mut file = File::open("messages.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let conversation: Vec<String> = contents
        .split("\n")
        .into_iter()
        .map(|val| val.to_string())
        .collect();

    // We tried to find the private keys by looking for collisions in the XOR.
    // Basically, we were hoping that by xoring the messages between them we could find
    // a long enough string of 0 so as to give us information about the private key.
    let mut chain_longest = 0;
    for (i, message) in conversation.iter().enumerate() {
        for other_message in conversation.iter().skip(i) {
            // Hex codes are superior to u128::MAX, so we parse them into a BigUint instead.
            let bits_1 = BigUint::parse_bytes(message.as_bytes(), 16).unwrap();
            let bits_2 = BigUint::parse_bytes(other_message.as_bytes(), 16).unwrap();

            // If messages have different sizes, we make sure to compare every bit with each other
            let mut bigger;
            let smaller;
            if message.len() > other_message.len() {
                bigger = bits_1;
                smaller = bits_2;
            } else {
                bigger = bits_2;
                smaller = bits_1;
            }

            // actual xoring logic, advancing 4 bytes at a time
            // because the key is in hex
            if smaller > BigUint::ZERO {
                while bigger > BigUint::ZERO {
                    let xor = bigger.clone() ^ smaller.clone();

                    let mut chain_current = 1;
                    for hex in xor.to_bytes_be().chunks(4) {
                        if hex == [0, 0, 0, 0] {
                            chain_current += 1;
                        } else if chain_current > chain_longest {
                            chain_longest = chain_current;
                        }
                    }

                    bigger >>= 4;
                }
            }
        }
    }

    println!("longest chain: {chain_longest}");
}
