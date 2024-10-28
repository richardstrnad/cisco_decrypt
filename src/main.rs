use core::panic;
use std::env::args;

const SALT: [u8; 53] = [
    100, 115, 102, 100, 59, 107, 102, 111, 65, 44, 46, 105, 121, 101, 119, 114, 107, 108, 100, 74,
    75, 68, 72, 83, 85, 66, 115, 103, 118, 99, 97, 54, 57, 56, 51, 52, 110, 99, 120, 118, 57, 56,
    55, 51, 50, 53, 52, 107, 59, 102, 103, 56, 55,
];

fn main() {
    let pw = args().skip(1).next();
    match pw {
        Some(pw) => println!("{}", decrypt(&pw).unwrap()),
        None => panic!("No password provided!"),
    }
}

fn decrypt(pw: &str) -> Result<String, String> {
    let index = pw[0..2]
        .parse::<usize>()
        .map_err(|_| "Invalid salt index".to_string())?;

    let enc_pw = &pw[2..];

    let hex_pw: Result<Vec<u8>, _> = (0..enc_pw.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&enc_pw[i..i + 2], 16))
        .collect();

    let hex_pw = hex_pw.map_err(|_| "Invalid hex in password".to_string())?;

    Ok(hex_pw
        .iter()
        .enumerate()
        .map(|(i, &hex_char)| (SALT[(i + index) % 53] ^ hex_char) as char)
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::decrypt;

    #[test]
    fn decrypt_test() {
        let decrypted = decrypt("12090404011C03162E").unwrap();
        assert_eq!(decrypted, "password".to_string());
    }
}
