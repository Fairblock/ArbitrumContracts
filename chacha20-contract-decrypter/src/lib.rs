
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::prelude::*;
use aead::Aead;
use chacha20poly1305::{
    aead::{ self, NewAead},
    ChaCha20Poly1305, Key, Nonce,
};
use hkdf::Hkdf;
use sha2::Sha256;
// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct DecrypterChacha20 {
       
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]

impl DecrypterChacha20 {
   
    fn decrypter(file_key: Vec<u8>, nonce: Vec<u8>, s: Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error> {
       let key =  stream_key(file_key.as_slice(), nonce.as_slice());
        let aead_key = Key::from_slice(key.as_slice());
        let a = ChaCha20Poly1305::new(aead_key);
        let nonce = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let plain = a.decrypt(&Nonce::from_slice(&nonce), &s[1..]).unwrap();
    
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