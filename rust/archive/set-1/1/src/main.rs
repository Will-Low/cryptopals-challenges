use hex;
use base64;
use std::env;

fn main() {
    let input: Vec<String> = env::args().collect();
    let bytes = hex_to_bytes(&input[1]);
    let base64 = bytes_to_base64(&bytes);
    println!("{:?}", base64); 
}

fn hex_to_bytes(input: &String) -> Vec<u8> {
    let hex = hex::decode(input);
    if let Ok(bytes) = hex {
        bytes
    } else {
        panic!()
    }
}

fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    base64::encode(bytes)
}
