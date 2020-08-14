/*
Convert hex to base64
The string:

49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
Should produce:

SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
So go ahead and make that happen. You'll need to use this code for the rest of the exercises.

Cryptopals Rule
Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.
*/

use hex;

fn main() {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex_to_bytes(hex_string);
    let b64 = bytes_to_b64(bytes);
    println!("{}", b64);
}

fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    hex::decode(hex_string).unwrap()
} 

fn bytes_to_b64(bytes: Vec<u8>) -> String{
    base64::encode(bytes)
}
