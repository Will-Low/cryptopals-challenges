/*
Single-byte XOR cipher
The hex encoded string:

1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.

Achievement Unlocked
You now have our permission to make "ETAOIN SHRDLU" jokes on Twitter.
*/

use hex;

fn main() {
    let string_1 = "1c0111001f010100061a024b53535009181c";
    let string_2 = "686974207468652062756c6c277320657965";
    let bytes_1 = hex_to_bytes(string_1);
    let bytes_2 = hex_to_bytes(string_2);
    let xored_bytes = xor_bytes(bytes_1, bytes_2);
    let xored_hex = bytes_to_hex(xored_bytes);
    println!("{}", xored_hex);
}

fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    hex::decode(hex_string).unwrap()
} 

fn xor_bytes(bytes_1: Vec<u8>, bytes_2: Vec<u8>) -> Vec<u8> {
    let mut xored = vec!();
    assert_eq!(bytes_1.len(), bytes_2.len(), "Uneven lengths");
    let length = bytes_1.len();
    for i in 0..length {
        xored.push(bytes_1[i] ^ bytes_2[i]);
    }
    xored
}

fn bytes_to_hex(bytes: Vec<u8>) -> String {
    hex::encode(bytes)
}
