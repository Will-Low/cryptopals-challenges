use hex;
use std::env;

fn main() {
    let input: Vec<String> = env::args().collect();
    let first_buffer_bytes = hex_to_bytes(&input[1]);
    let second_buffer_bytes = hex_to_bytes(&input[2]);
    let xored = xor(first_buffer_bytes, second_buffer_bytes);
    println!("{:?}", hex::encode(xored));
}

fn hex_to_bytes(input: &String) -> Vec<u8> {
    let hex = hex::decode(input);
    if let Ok(bytes) = hex {
        bytes
    } else {
        panic!()
    }
}

fn xor(first_buffer: Vec<u8>, second_buffer: Vec<u8>) -> Vec<u8> {
    first_buffer
        .iter()
        .zip(second_buffer.iter())
        .map(|(&byte1, &byte2)| byte1 ^ byte2)
        .collect()
}
