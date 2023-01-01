use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

pub fn encode_password(password: String) -> String {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();

    let mut salt = [0u8; CREDENTIAL_LEN];

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    HEXUPPER.encode(&pbkdf2_hash)
}

pub fn verify_password(password: &str) -> String{
    println!("the password that entered is: {}", &password);
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();

    let mut salt = [0u8; CREDENTIAL_LEN];

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    let should_succeed = pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    HEXUPPER.encode(&pbkdf2_hash)
}