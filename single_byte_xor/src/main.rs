extern crate rustc_serialize as serialize;

use serialize::hex::FromHex;
use std::str;

fn main() {
    let tuple = decode_string("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string());
    println!("{}: {}", tuple.0, tuple.1);
}

fn decode_string(hex_string: std::string::String) -> (u8, std::string::String) {
    let max_char = 128;

    let hex = hex_string.from_hex().unwrap();

    let mut best_decoded_string = std::string::String::new();
    let mut best_xor_char = 0;

    for xor_char in 0..max_char {
        let mut xor_vec = Vec::new();
        for byte in hex.iter() {
            xor_vec.push(byte ^ xor_char);
        }

        let decoded_str = str::from_utf8(&xor_vec).unwrap();
        if compare_string(best_decoded_string.to_string(), decoded_str.to_string()) == true {
            best_decoded_string = decoded_str.to_string();
            best_xor_char = xor_char;
        }
    }
    return (best_xor_char, best_decoded_string);
}

fn score_string(s: &str) -> usize {
    return s.matches(char::is_alphabetic).count() + s.matches(' ').count();
}

fn compare_string<'a>(s1: std::string::String, s2: std::string::String) -> bool {
    let s1_score = score_string(&s1);
    let s2_score = score_string(&s2);

    if s1_score > s2_score {
        return false;
    } else {
        return true;
    }
}
