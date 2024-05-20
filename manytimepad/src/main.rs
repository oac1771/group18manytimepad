use hex;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let messages = read_messages();

    let lengths: Vec<usize> = messages.into_iter().map(|val| val.len()).collect();


    println!("{:?}", lengths);

    // for message in messages {
    //     println!("{:?}", hex::decode(message).unwrap())
    // }

}


fn find_length(messages: Vec<String>) {
    let lengths: Vec<usize> = messages.into_iter().map(|val| val.len()).collect();
}

fn read_messages()-> Vec<String> {
    let mut file = File::open("messages.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let result: Vec<String> = contents.split("\n").into_iter().map(|val| val.to_string()).collect();

    return result
}