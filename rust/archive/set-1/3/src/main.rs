use hex;
use std::env;
use std::str;

fn main() {
    let input: Vec<String> = env::args().collect();
    let bytes = hex_to_bytes(&input[1]);
    let xored_vectors = xor_with_letters(&bytes);
    let decrypted_ciphertext = determine_ciphertext(xored_vectors);
    println!("{:?}", str::from_utf8(&decrypted_ciphertext));
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

fn determine_ciphertext(xored_vectors: Vec<Vec<u8>>) -> Vec<u8> {
    let mut scores: Vec<(u8, Vec<u8>)> = vec![];
    for possible_text in xored_vectors {
        let mut score = 0;
        for letter in &possible_text {
            if *letter as char == 'e' 
                || *letter as char == 'E' 
                || *letter as char == 't' 
                || *letter as char == 'T'
                || *letter as char == 'a'
                || *letter as char == 'A' 
                || *letter as char == 'o'
                || *letter as char == 'O'
                || *letter as char == 'i' 
                || *letter as char == 'I' 
                || *letter as char == 'n'
                || *letter as char == 'N' 
                || *letter as char == 's'
                || *letter as char == 'S'
                || *letter as char == 's'
                || *letter as char == 'S' {
                score = score + 1;    
            }
        }
        //println!("{:?}, {}", score, str::from_utf8(&possible_text).unwrap()); 
        scores.push((score, possible_text));
    }
    let mut max: (u8, Vec<u8>) = (0, vec![]);
    for each in scores {
        if each.0 > max.0 {
            max = each;
        }
    }
    max.1
}
