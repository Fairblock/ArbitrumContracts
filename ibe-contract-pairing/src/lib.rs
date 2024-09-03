
#![cfg_attr(not(feature = "export-abi"), no_main)]

extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.

use sha2::Digest;
use ic_bls12_381::{G1Affine,G2Affine,pairing};




use stylus_sdk::{alloy_primitives::hex::ToHexExt, prelude::sol_storage, stylus_proc::{entrypoint, external}};


sol_storage! {
    #[entrypoint]
    pub struct IBEPairing {
       
    }
}


#[external]
impl IBEPairing {
  
    pub fn pairing(private: Vec<u8>, cu : Vec<u8>) -> Vec<u8>{

        let mut hash = sha2::Sha256::new();
        let _cu = G1Affine::from_compressed(&cu.try_into().unwrap()).unwrap();
        let pr = G2Affine::from_compressed(&private.try_into().unwrap()).unwrap();
        let r_gid =  pairing(&_cu, &pr);
       // let array_data: [u8; 576] = r_gid.to_bytes().to_vec().try_into().unwrap();
        hash.update(b"IBE-H2");
        hash.update(r_gid.to_bytes().to_vec());
        let h_r_git: &[u8] = &hash.finalize().to_vec()[0..32];
        let out : [u8;32] = h_r_git.try_into().unwrap();
       return out.to_vec();
     
}



}

