extern crate itertools;
use itertools::Itertools;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|ch| ch.is_ascii() && ch.is_alphanumeric())
        .enumerate()
        .map(|(_, ch)| {
            let ch = ch.to_lowercase().collect::<Vec<char>>()[0];
            if !ch.is_alphabetic() {
                return ch;
            }
            (122 - ch as u8 + 97) as char
        })
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .enumerate()
        .map(|(_, ch)| {
            if !ch.is_alphabetic() {
                return ch;
            }
            (122 - ch as u8 + 97) as char
        })
        .collect::<String>()
}
