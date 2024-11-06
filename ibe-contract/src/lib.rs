#![cfg_attr(not(feature = "export-abi"), no_main)]
#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::FromStr;
use sha2::Digest;
use stylus_sdk::function_selector;
use stylus_sdk::prelude::sol_interface;
use stylus_sdk::storage::{StorageBool, StorageAddress};
use stylus_sdk::{alloy_primitives::Address, alloy_sol_types, call::Call};
use stylus_sdk::{
    prelude::sol_storage,
    stylus_proc::{entrypoint, external},
};

const BLOCK_SIZE: usize = 32;

sol_storage! {
    #[entrypoint]
    pub struct IBE {
     StorageAddress hasher_addr;
     StorageBool initialized;
    }
}

sol_interface! {

    interface IHasher {
        function verify(uint8[] memory sigma, uint8[] memory msg, uint8[] memory cu) external view returns (bool);
    }
}

/// Performs the IBE decryption
/// The initialize() function can be called once to set the address of the hasher contract
/// 
/// decrypt() function: 
/// # Parameters 
///
/// - `r_gid`: A `Vec<u8>` containing the pairing of cu and decryption key.
/// - `cv`: A `Vec<u8>` containing the cv part from ciphertext.
/// - `cw`: A `Vec<u8>` containing the cw part from ciphertext.
/// - `cu`: A `Vec<u8>` containing the cu part from ciphertext.
///
/// # Returns
///
/// - `Ok(Vec<u8>)`: If successful, returns a `Vec<u8>` containing the plaintext.
/// - `Err(stylus_sdk::call::Error)`: If an error occurs during decryption, it returns an error from the `stylus_sdk::call::Error` type.
#[external]
impl IBE {
    pub fn initialize(&mut self, hasher_addr: String) -> Result<(), stylus_sdk::call::Error> {
        let initialized = self.initialized.get();
        if initialized {
            return Err(stylus_sdk::call::Error::Revert(
                "Already initialized".as_bytes().to_vec(),
            ));
        }
        self.hasher_addr.set(Address::from_str(&hasher_addr)
        .map_err(|_| {
            stylus_sdk::call::Error::Revert("Invalid hasher address".as_bytes().to_vec())
        })?);
        self.initialized.set(true);
        return Ok(());
    }
    pub fn decrypt(
        &mut self,
        r_gid: Vec<u8>,
        cv: Vec<u8>,
        cw: Vec<u8>,
        cu: Vec<u8>,
    ) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        if cu.len() != 48 || cv.len() > BLOCK_SIZE || cw.len() > BLOCK_SIZE {
            return Err(stylus_sdk::call::Error::Revert(
                "Invalid input length".as_bytes().to_vec(),
            ));
        }

        let sigma = {
            let mut hash = sha2::Sha256::new();

            hash.update(b"IBE-H2");
            hash.update(r_gid);

            let h_r_git: &[u8] = &hash.finalize().to_vec()[0..32];

            xor(h_r_git, &cv)
        };

        let msg = {
            let mut hash = sha2::Sha256::new();
            hash.update(b"IBE-H4");
            hash.update(&sigma);
            let h_sigma = &hash.finalize()[0..BLOCK_SIZE];
            xor(h_sigma, &cw)
        };

        let hasher = IHasher {
            address: *self.hasher_addr,
        };
        let verify_res = hasher
            .verify(Call::new(), sigma.clone(), msg.clone(), cu)
            .map_err(|_| stylus_sdk::call::Error::Revert("Hasher error".as_bytes().to_vec()))?;

        if !verify_res {
            return Err(stylus_sdk::call::Error::Revert(
                "Verfication failed".as_bytes().to_vec(),
            ));
        }

        Ok(msg)
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}
