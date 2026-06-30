use core::time;
use std::time::SystemTime;

use base32::Alphabet;
use totp_rs::{Algorithm::SHA1, Rfc6238, TOTP};

pub enum Algorithms {
    Sha1,
    Sha256,
    Sha512,
}

//Hardcode defaults for now, need to test totp more
pub fn generate_token(secret_string: String) -> String {
    let secret: Vec<u8> = base32::decode(Alphabet::Rfc4648 { padding: false }, &secret_string)
        .expect("Invalid Base32 string");
    let generator = TOTP::new_unchecked(SHA1, 6, 1, 30, secret);

    let start = SystemTime::now();

    let time_as_u64 = start
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    generator.generate(time_as_u64)
}
