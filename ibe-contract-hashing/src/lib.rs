#![cfg_attr(not(feature = "export-abi"), no_main)]

extern crate alloc;
use sha2::Digest;
use ic_bls12_381::{G1Affine, G1Projective, Scalar};
use num_bigint::{BigInt, Sign};
use stylus_sdk::{
    prelude::{public, sol_storage},
    stylus_proc::entrypoint,
};

sol_storage! {
    #[entrypoint]
    pub struct Hasher {

    }
}

/// Calculates the hash over sigma and message and comapres it with the cu part from the ciphertext
///
/// # Parameters
///
/// - `sigma`: A `Vec<u8>` containing the sigma calculated in the ibe-contract.
/// - `msg`: A `Vec<u8>` containing the decrypted message.
/// - `cu`: A `Vec<u8>` containing the cu part from ciphertext.
///
/// # Returns
///
/// - `Ok(bool)`: If equal, returns true. 
/// - `Err(stylus_sdk::call::Error)`: If an error occurs during decryption, it returns an error from the `stylus_sdk::call::Error` type.
#[public]
impl Hasher {
    pub fn verify(
        sigma: Vec<u8>,
        msg: Vec<u8>,
        cu: Vec<u8>,
    ) -> Result<bool, stylus_sdk::call::Error> {
        if sigma.len() != 32 || msg.len() != 32 || cu.len() != 48 {
            return Err(stylus_sdk::call::Error::Revert("Invalid input length".as_bytes().to_vec()));  
        }

        let r_g = {
            let r = h3(sigma.to_vec(), msg.to_vec())?;
            let rs_ct = Scalar::from_bytes(&r.try_into().unwrap());
            if rs_ct.is_some().unwrap_u8() == 0 {
               return Err(stylus_sdk::call::Error::Revert("Error deserializing the scalar".as_bytes().to_vec()));
               
            }
            let rs = rs_ct.unwrap();
            let g1_base_projective = G1Projective::from(G1Affine::generator());
            g1_base_projective * rs
        };

        let result_affine = G1Affine::from(r_g);
        Ok(result_affine.to_compressed().to_vec() == cu)
    }
}

pub fn h3(sigma: Vec<u8>, msg: Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"IBE-H3");
    hasher.update(&sigma);
    hasher.update(&msg);
    let initial_hash = hasher.finalize_reset();

    for i in 1..=65535u16 {
        hasher.update(&i.to_le_bytes());
        hasher.update(&initial_hash);
        let mut hashed = hasher.finalize_reset().to_vec();

        hashed[0] /= 2;
        hashed.reverse();

        let scalar_option = Scalar::from_bytes(&hashed[..32].try_into().unwrap());
        if scalar_option.is_some().into() {
            return Ok(hashed[..32].to_vec());
        }
    }

    Err(stylus_sdk::call::Error::Revert("Hashing error".as_bytes().to_vec()))
}
