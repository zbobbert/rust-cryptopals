extern crate rustc_serialize as serialize;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use serialize::hex::FromHex;
use std::str;

fn main() {
    // Open the path in read-only mode, returns `io::Result<File>`
        let file = match File::open("4.txt") {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open 4.txt - {}", why),
            Ok(file) => file,
        };

    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let tuple = decode_string(line.unwrap());

        //Filter out lines that contain invalid utf-8 replacement characters.
        if !tuple.1.contains("�") {
            println!("Line {}: {}: {}", index, tuple.0, tuple.1);
        }
    }
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

        let decoded_str = String::from_utf8_lossy(&xor_vec);
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
