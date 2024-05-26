use array_bytes::hex2bytes_unchecked;
use std::collections::HashMap;
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

    let encrypted_messages = conversation
        .clone()
        .into_iter()
        .map(|line| hex::decode(line).expect("Invalid hex in file"))
        .collect::<Vec<Vec<u8>>>();

    let decrypted_messages = decrypt_messages(&encrypted_messages);
    for (i, messages) in decrypted_messages {
        println!("Clue for message {}: {:?}", i + 1, messages);
    }

    let key = "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.";
    for (i, ciphertext) in conversation.into_iter().enumerate() {
        let message = decrypt(key.as_bytes(), ciphertext.as_str());
        println!("Message {}: {:?}", i + 1, message);
    }
}

fn decrypt_messages(encrypted_messages: &[Vec<u8>]) -> HashMap<usize, String> {
    let mut decrypted_messages: HashMap<usize, String> = HashMap::new();

    for i in 0..encrypted_messages.len() {
        let mut decrypted_message = vec![95u8; encrypted_messages[i].len()];

        for j in 0..encrypted_messages.len() {
            if i != j {
                let xor_result = xor_bytes(&encrypted_messages[i], &encrypted_messages[j]);

                for (k, &byte) in xor_result.iter().enumerate() {
                    if byte.is_ascii_alphabetic() || byte == b' ' {
                        decrypted_message[k] = byte;
                    }
                }
            }
        }

        let decrypted_string = String::from_utf8_lossy(&decrypted_message).to_string();
        decrypted_messages.insert(i, decrypted_string);
    }

    decrypted_messages
}

fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

pub fn decrypt(key: &[u8], cipher: &str) -> String {
    let cipher = hex2bytes_unchecked(cipher);
    String::from_utf8_lossy(&xor_bytes(key, &cipher)).to_string()
}
