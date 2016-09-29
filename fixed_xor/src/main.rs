extern crate rustc_serialize as serialize;

use serialize::hex::FromHex;
use serialize::hex::ToHex;

fn main() {
    let str1 = "1c0111001f010100061a024b53535009181c";
    let str2 = "686974207468652062756c6c277320657965";

    let bytes1 = str1.from_hex().unwrap();
    let bytes2 = str2.from_hex().unwrap();
    let mut xor = Vec::new();

    let mut i = 0;
    for byte in bytes1 {
        xor.push(byte ^ bytes2[i]);
        i = i + 1;
    }

    println!("{}", xor.to_hex());
}
