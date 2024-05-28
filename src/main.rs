mod vigenere;
use vigenere::*;

mod substitution;
use substitution::*;
//TWENTYFOUR
//KEY: BDAY
//ENCRYPTED: UZELUBFMVU
fn main() {
    let key_size = 7; 
    let plain_text = "IAMGOINGTOKILLPRESIDENTBIDENONJULYFITH";
    let key = "GARDNER";
    let cipher_text = "QQPRNGKSS";

    let encrpyed_blue = encrypt(plain_text, key);

    KRYPTOS(key_size, plain_text, &encrpyed_blue)


    //KRYPTOS(key_size, plain_text, cipher_text);
}

pub fn KRYPTOS(size: usize, plain_text: &str, cipher_text: &str) {
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut combination = vec!['A'; size];
    loop {
        let key = combination.iter().collect::<String>();

        let decrpyted = decrypt(&cipher_text, key.as_str());
        if decrpyted == plain_text.to_string() || is_substitution_cipher(plain_text, cipher_text) {
            println!("key: {} ... plain text: {} ... Decrpyted: {} ", key, plain_text, decrpyted);
        }
        let mut carry = true;
        for i in (0..size).rev() {
            if combination[i] < 'Z' {
                combination[i] = ((combination[i] as u8) + 1) as char;
                carry = false;
                break;
            } else {
                combination[i] = 'A';
            }
        }

        if carry {
            break;
        }
    }
}