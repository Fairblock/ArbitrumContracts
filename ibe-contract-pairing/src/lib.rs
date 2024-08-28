
#![cfg_attr(not(feature = "export-abi"), no_main)]
#![no_std]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
use ic_bls12_381::{G1Affine,G2Affine,pairing};
use alloc::vec::Vec;

use stylus_sdk::alloy_primitives::ruint::aliases::B256;
use stylus_sdk::alloy_primitives::B512;
use stylus_sdk::{alloy_primitives::B128, alloy_sol_types};


use stylus_sdk::{alloy_primitives::hex::ToHexExt, prelude::sol_storage, stylus_proc::{entrypoint, external}};


sol_storage! {
    #[entrypoint]
    pub struct IBEPairing {
       
    }
}


#[external]
impl IBEPairing {
   
    pub fn pairing(&self, private: Vec<u8>, cu : Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error>{

     
        let _cu = G1Affine::from_compressed(&cu.clone().try_into().unwrap()).unwrap();
        let pr = G2Affine::from_compressed(&private.try_into().unwrap()).unwrap();

        let r_gid =  pairing(&_cu, &pr);
        Ok(r_gid.to_bytes().to_vec())


}



}

