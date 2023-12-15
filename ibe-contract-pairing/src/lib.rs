
#![cfg_attr(not(feature = "export-abi"), no_main)]
#![no_std]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



use alloc::vec::Vec;
// use bls12_381_plus::G1Affine;
use ic_bls12_381::{G1Affine, G2Affine, pairing};


/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{prelude::sol_storage, stylus_proc::{entrypoint, external}};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct IBEPairing {
       
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl IBEPairing {
   
    pub fn pairing(&self, private: Vec<u8>, cu : Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error>{

     
            let cu = G1Affine::from_compressed(&cu.try_into().unwrap()).unwrap();
            let pr = G2Affine::from_compressed(&private.try_into().unwrap()).unwrap();
            //let prepared = G2Prepared::from(pr);
            let r_gid = pairing(&cu, &pr);
       
            Ok(r_gid.to_bytes().to_vec())
    
   
    }

}

