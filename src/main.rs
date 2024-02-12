use std::collections::HashMap;
use clap::Parser;

fn beaverscratch_decrypt_hashmap() -> HashMap<char, char> {
    let mut hm: HashMap<char, char> = HashMap::new();
    hm.insert('a', 'b');
    hm.insert('b', 'a');
    hm.insert('c', 'd');
    hm.insert('d', 'c');
    hm.insert('e', 'f');
    hm.insert('f', 'e');
    hm.insert('g', 'g');
    hm.insert('h', 'h');
    hm.insert('i', 'j');
    hm.insert('j', 'i');
    hm.insert('k', 'l');
    hm.insert('l', 'k');
    hm.insert('m', 'n');
    hm.insert('n', 'm');
    hm.insert('o', 'p');
    hm.insert('p', 'o');
    hm.insert('q', 'r');
    hm.insert('r', 'q');
    hm.insert('s', 't');
    hm.insert('t', 's');
    hm.insert('u', 'v');
    hm.insert('v', 'u');
    hm.insert('w', 'x');
    hm.insert('x', 'w');
    hm.insert('y', 'z');
    hm.insert('z', 'y');
    
    hm
}

fn beaverscratch_encrypt_hashmap() -> HashMap<char, char> {
    let mut hm: HashMap<char, char> = HashMap::new();
    hm.insert('b', 'a');
    hm.insert('a', 'b');
    hm.insert('d', 'c');
    hm.insert('c', 'd');
    hm.insert('f', 'e');
    hm.insert('e', 'f');
    hm.insert('h', 'g');
    hm.insert('g', 'h');
    hm.insert('j', 'i');
    hm.insert('i', 'j');
    hm.insert('l', 'k');
    hm.insert('k', 'l');
    hm.insert('n', 'm');
    hm.insert('m', 'n');
    hm.insert('o', 'o');
    hm.insert('p', 'p');
    hm.insert('r', 'q');
    hm.insert('q', 'r');
    hm.insert('t', 's');
    hm.insert('s', 't');
    hm.insert('v', 'u');
    hm.insert('u', 'v');
    hm.insert('x', 'w');
    hm.insert('w', 'x');
    hm.insert('z', 'y');
    hm.insert('y', 'z');
    
    hm
}

/// Simple program to translate to and from beaverscratch
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Text to be decrypted/encrypted
    #[arg(short, long)]
    text: String,

    /// Encrypt given text (Decrypt by default)
    #[arg(short, long, default_value_t = false)]
    encrypt: bool,
}

fn main() -> Result<(), u8> {
    let args = Args::parse();
    let mut output = String::new();

    if !args.encrypt {
        // decrypt
        println!("---- Decrypting ----");
        let hm: HashMap<char, char> = beaverscratch_decrypt_hashmap();
        args.text.chars().for_each(|char| {
            if !char.is_alphabetic() {
                output.push(char);
            }
            else if char.is_uppercase() {
                output.push(hm.get(&char.to_ascii_lowercase())
                   .unwrap_or(&' ')
                   .to_ascii_uppercase());
            } else {
                output.push(*hm.get(&char)
                   .unwrap_or(&' '));
            }
        });
    } else {
        // encrypt
        println!("---- Encrypting ----");
        let hm: HashMap<char, char> = beaverscratch_encrypt_hashmap();
        args.text.chars().for_each(|char| {
            if !char.is_alphabetic() {
                output.push(char);
            }
            else if char.is_uppercase() {
                output.push(hm.get(&char.to_ascii_lowercase())
                   .unwrap_or(&' ')
                   .to_ascii_uppercase());
            } else {
                output.push(*hm.get(&char)
                   .unwrap_or(&' '));
            }
        });
    }

    println!("Result: {}", output);

    Ok(())
}
