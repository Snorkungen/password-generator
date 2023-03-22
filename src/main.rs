/*
    I S**k
    Knowing what your actions lead to is something. yes?

// https://www.security.org/how-secure-is-my-password/

*/

use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Master string used when generating a password
    #[arg(short, long)]
    master: String,

    /// Key string used when generating a password
    #[arg(short, long)]
    key: String,
}

// ascii char range 33 -> 126
const PWD_CHARS :&str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
const PASWORD_LENGTH: usize = 26;

fn main() {
    let args = Args::parse();

    let pwd_chars = generate_pwd_chars(&args.master);
    let pwd = generate_pwd(&pwd_chars, &args.key);
    println!("{}", pwd);
}

fn generate_pwd_chars(master: &String) -> Vec<char> {
    let mut chars: Vec<char> = PWD_CHARS.chars().collect();

    let mut rng: Pcg64 = Seeder::from(master).make_rng();
    chars.shuffle(&mut rng);

    return chars;
}

fn generate_pwd(pwd_chars: &Vec<char>, key: &String) -> String {
    let mut pwd = String::from("");

    let mut rng: Pcg64 = Seeder::from(key).make_rng();
    let range = 1..=PWD_CHARS.len();
    let start_index = rng.gen_range(range.clone());

    for _ in 0..PASWORD_LENGTH {
        let i = start_index * rng.gen_range(range.clone());
        pwd.push(pwd_chars[i % pwd_chars.len()]);
    }

    return pwd;
}