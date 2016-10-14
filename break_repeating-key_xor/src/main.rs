extern crate rustc_serialize as serialize;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use serialize::base64::FromBase64;
use std::collections::HashMap;

fn main() {
    //let test1 = "this is a test";
    //let test2 = "wokka wokka!!!";
    //println!("{}", hamming_distance(test1.as_bytes(), test2.as_bytes()));

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open("6.txt") {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open 6.txt - {}", why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);

    let keysize_min = 2;
    let keysize_max = 40;
    let mut key_totals = HashMap::new();

    let mut file_string = std::string::String::new();
    let _ = reader.read_to_string(&mut file_string);
    let line_vector = file_string.from_base64().unwrap();
    //println!("{:?}", line_vector.to_hex());
    for keysize in keysize_min..keysize_max {
        let this_distance = keysize_hamming_distance(keysize, &line_vector);
        let this_distance_total = key_totals.entry(keysize).or_insert(0.0);
        *this_distance_total += this_distance;
    }

    let mut likely_keysize_score = 0.0;
    let mut likely_keysize = 0;

    for keysize in keysize_min..keysize_max {
        let this_keysize_total = *key_totals.get(&keysize).unwrap();
        //println!("{}: {}", keysize, this_keysize_total);
        if likely_keysize_score == 0.0 {
            likely_keysize_score = this_keysize_total;
            likely_keysize = keysize;
        } else if this_keysize_total < likely_keysize_score {
            likely_keysize_score = this_keysize_total;
            likely_keysize = keysize;
        }
    }
    //println!("{}: {}", likely_keysize, likely_keysize_score);

    let mut block_map = HashMap::new();
    let mut line_iterator = line_vector.iter().peekable();
    while line_iterator.peek() != None {
        let mut block = std::vec::Vec::new();
        read_keysize_bytes(likely_keysize, &mut line_iterator, &mut block);
        //println!("------- Block: {:?}", block);
        for keysize_index in 0..likely_keysize {
            let mut working_block = block_map.entry(keysize_index).or_insert(std::vec::Vec::new());
            let byte_option = block.get(keysize_index as usize);
            if byte_option != None {
                working_block.push(*byte_option.unwrap());
                //println!("Working Block: {:?}", working_block);
            }
            else {
                break;
            }
        }
    }

    let mut likely_keys = HashMap::new();
    for transpose_index in block_map {
        let decoded = decode_vec(&transpose_index.1);
        //println!("Attempt {:?}: {:?}", transpose_index, decoded);
        likely_keys.insert(transpose_index.0, decoded);
    }

    let block_length = likely_keys.get(&0).unwrap().1.len() as u8;
    let mut untransposed_vec = std::vec::Vec::new();
    //println!("{:?}", block_length);
    for i in 0..likely_keysize {
        for block_index in 0..block_length {
            //println!("{:?},{}", i, block_index);
            let byte_option = likely_keys.get(&block_index);
            if byte_option != None {
                untransposed_vec.push(byte_option.unwrap().1.as_bytes()[i as usize]);
            }
            else {
                break;
            }
        }
    }
    let decoded_str = String::from_utf8_lossy(&untransposed_vec);
    println!("{:?}", decoded_str);
}

fn hamming_distance(s1: &[u8], s2: &[u8]) -> f32 {
    let mut distance = 0.0;
    for byte in s1.iter().zip(s2.iter()) {
        //println!("{:b}^{:b}", byte.0, byte.1);
        distance = distance + (byte.0 ^ byte.1).count_ones() as f32;
    }
    return distance;
}

fn keysize_hamming_distance(keysize: u8, msg: &std::vec::Vec<u8>) -> f32 {
    let mut distance = 0.0;
    let mut iter_count = 0;
    let mut msg_bytes = msg.iter().peekable();
    while msg_bytes.peek() != None {
        let mut keysize_bytes1 = std::vec::Vec::new();
        read_keysize_bytes(keysize, &mut msg_bytes, &mut keysize_bytes1);
        if msg_bytes.peek() == None {
            break;
        }
        let mut keysize_bytes2 = std::vec::Vec::new();
        read_keysize_bytes(keysize, &mut msg_bytes, &mut keysize_bytes2);
        //println!("{:?}^{:?}={}", keysize_bytes1.as_slice(), keysize_bytes2.as_slice(), hamming_distance(keysize_bytes1.as_slice(), keysize_bytes2.as_slice()));
        distance += hamming_distance(keysize_bytes1.as_slice(), keysize_bytes2.as_slice()) / keysize as f32;
        //distance = distance + (((keysize_bytes1 ^ keysize_bytes2).count_ones()) as f32 / keysize as f32);
        iter_count += 1;
    }
    return distance/iter_count as f32;
}

fn read_keysize_bytes(keysize: u8, msg_iter: &mut std::iter::Peekable<std::slice::Iter<u8>>, keysize_bytes: &mut std::vec::Vec<u8>) {
    for _ in 0..keysize {
        if msg_iter.peek() == None {
            break;
        }
        keysize_bytes.push(*msg_iter.next().unwrap());
        //println!("{:b}", keysize_bytes1.last().unwrap());
    }
}

//Borrowed from detect_single_character_xor

fn decode_vec(vec: &std::vec::Vec<u8>) -> (u8, std::string::String) {
    let max_char = 128;

    let mut best_decoded_string = std::string::String::new();
    let mut best_xor_char = 0;

    for xor_char in 0..max_char {
        let mut xor_vec = Vec::new();
        for byte in vec.iter() {
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
