pub fn generate_key_stream(key: &str, length: usize) -> String {
    key.chars().cycle().take(length).collect()
}

pub fn encrypt(plaintext: &str, key: &str) -> String {
    let key_stream = generate_key_stream(key, plaintext.len());
    plaintext
        .chars()
        .zip(key_stream.chars())
        .map(|(p, k)| {
            let shift = k as u8 - b'A';
            let base = if p.is_ascii_uppercase() { b'A' } else { b'a' };
            ((p as u8 - base + shift) % 26 + base) as char
        })
        .collect()
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let key_stream = generate_key_stream(key, ciphertext.len());
    ciphertext
        .chars()
        .zip(key_stream.chars())
        .map(|(c, k)| {
            let shift = k as u8 - b'A';
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            ((c as u8 - base + 26 - shift) % 26 + base) as char
        })
        .collect()
}
