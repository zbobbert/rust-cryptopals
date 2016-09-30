extern crate rustc_serialize as serialize;

use serialize::hex::ToHex;

fn main() {
    let msg = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let key = "ICE";
    println!("{}", repeating_key_xor(key, msg));
}

fn repeating_key_xor<'a>(key: &str, msg: &str) -> std::string::String{
    let mut key_iterator = key.chars().cycle();
    let mut encrypted = String::new();
    for c in msg.chars() {
        encrypted.push((c as u8 ^ key_iterator.next().unwrap() as u8) as char);
    }
    return encrypted.as_bytes().to_hex();
}
