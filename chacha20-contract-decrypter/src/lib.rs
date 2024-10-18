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

/// Performs the symmetric key decryption.
///
/// # Parameters
///
/// - `key`: A `Vec<u8>` containing the symmetric key. It should be 32 bytes.
/// - `nonce`: A `Vec<u8>` containing the nonce. It should be 16 bytes.
/// - `ciphertext`: A `Vec<u8>` representing the ciphertext to be decrypted.
///
/// # Returns
///
/// - `Ok(Vec<u8>)`: If successful, returns a `Vec<u8>` containing the plaintext.
/// - `Err(stylus_sdk::call::Error)`: If an error occurs during decryption, it returns an error from the `stylus_sdk::call::Error` type.
#[public]
impl DecrypterChacha20 {
    fn decrypter(
        key: Vec<u8>,
        nonce: Vec<u8>,
        ciphertext: Vec<u8>,
    ) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        if key.len() != 32 || nonce.len() != 16 || ciphertext.len() < 2 {
            return Err(stylus_sdk::call::Error::Revert("Wrong input length".as_bytes().to_vec()));
        }
        let key = stream_key(key.as_slice(), nonce.as_slice());
        let aead_key = Key::from_slice(key.as_slice());
        let chacha20 = ChaCha20Poly1305::new(aead_key);
        let n = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let plain = chacha20
            .decrypt(&Nonce::from_slice(&n), &ciphertext[0..])
            .map_err(|_| stylus_sdk::call::Error::Revert("decryption error".as_bytes().to_vec()))?;

        Ok(plain)
    }
}

fn stream_key(key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let h = Hkdf::<Sha256>::new(Some(nonce), key);
    let mut stream_key = vec![0u8; 32];

    h.expand(b"payload", &mut stream_key)
        .expect("age: internal error: failed to read from HKDF");

    stream_key
}
