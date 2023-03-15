/*
    I S**k


// https://www.security.org/how-secure-is-my-password/

*/

use std::str;
use rand::prelude::*;

// ascii char range 33 -> 126
const CHARS :&str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
const PASWORD_LENGTH: usize = 28;

fn main() {
    let master: String = "Master".to_string();
    let key: String = "yusdggds-".to_string();

    // let mut pwd = "".to_string();

    let mut chars: Vec<&str> = CHARS.split("").filter(|s| s != &"").collect();

    // println!("{:?}", chars);

    // shuffle_chars(&mut chars,&master);

    // println!("{:?}", chars);

    // println!("{}", pwd)

    let pwd = generate_pwd(&chars, &key);
}

fn shuffle_chars(chars: &mut Vec<&str>, master: &String) {
    panic!("Not implemented")
}

fn generate_pwd(chars: &Vec<&str>, key: &String) -> String {
    let mut pwd = String::new();
    let key_vec: Vec<&str> = key.split("").collect();

    for i in 1..PASWORD_LENGTH {
        let mut p = i + 1;

        if key_vec[p % key_vec.len()] > chars[p % chars.len()] {
            p *= p;
        }

        pwd.push_str(chars[p % chars.len()]);
    }

    pwd
}
