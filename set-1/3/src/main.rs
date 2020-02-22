use hex;
use std::env;

fn main() {
    let input: Vec<String> = env::args().collect();
    let bytes = hex_to_bytes(&input[1]);
    let xored_vectors = xor_with_letters(&bytes);
    let 
    println!("{:?}", xored_vectors);
}

fn hex_to_bytes(input: &String) -> Vec<u8> {
    let hex = hex::decode(input);
    if let Ok(bytes) = hex {
        bytes
    } else {
        panic!()
    }
}

fn xor_with_letters(bytes: &Vec<u8>) -> Vec<Vec<u8>>{
    let alphabet = b"abcdefghijklmnopqrstuvwxyz";
    let mut xored_vectors = vec![];
    for letter in alphabet {
        let letter_vec = vec![letter; 26];

        let xored_letter_vector = bytes
            .iter()
            .zip(letter_vec.iter())
            .map(|(&byte1, &byte2)| byte1 ^ byte2)
            .collect();
        xored_vectors.push(xored_letter_vector);
    }
    xored_vectors
}
