use std::collections::HashMap;

pub fn is_substitution_cipher(plaintext: &str, ciphertext: &str) -> bool {
    if plaintext.len() != ciphertext.len() {
        return false;
    }

    let mut char_map = HashMap::new();
    let mut used_chars = HashMap::new();

    for (p, c) in plaintext.chars().zip(ciphertext.chars()) {
        if let Some(&mapped_char) = char_map.get(&p) {
            if mapped_char != c {
                return false;
            }
        } else {
            if used_chars.contains_key(&c) {
                return false;
            }
            char_map.insert(p, c);
            used_chars.insert(c, ());
        }
    }

    true
}

fn main() {

}