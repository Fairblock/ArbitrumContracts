#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use aead::Aead;
use chacha20poly1305::{
    aead::{self, NewAead},
    ChaCha20Poly1305, Key, Nonce,
};
use hkdf::Hkdf;
use sha2::Sha256;
use stylus_sdk::prelude::*;

sol_storage! {
    #[entrypoint]
    pub struct DecrypterChacha20 {

    }
}

#[external]

impl DecrypterChacha20 {
    fn decrypter(
        file_key: Vec<u8>,
        nonce: Vec<u8>,
        s: Vec<u8>,
    ) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        if file_key.len() != 32 || nonce.len() != 16 || s.len() < 2 {
            return Err(stylus_sdk::call::Error::Revert(vec![1]));
        }
        let key = stream_key(file_key.as_slice(), nonce.as_slice());
        let aead_key = Key::from_slice(key.as_slice());
        let a = ChaCha20Poly1305::new(aead_key);
        let nonce = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let plain = a
            .decrypt(&Nonce::from_slice(&nonce), &s[1..])
            .map_err(|_| stylus_sdk::call::Error::Revert(vec![2]))?;

        Ok(plain)
    }
}

fn stream_key(file_key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let h = Hkdf::<Sha256>::new(Some(nonce), file_key);
    let mut stream_key = vec![0u8; 32];

    h.expand(b"payload", &mut stream_key)
        .expect("age: internal error: failed to read from HKDF");

    stream_key
}
