
#![cfg_attr(not(feature = "export-abi"), no_main)]

extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.

use sha2::Digest;
use ic_bls12_381::{G1Affine,G2Affine,pairing};




use stylus_sdk::{ prelude::sol_storage, stylus_proc::{entrypoint, external}};


sol_storage! {
    #[entrypoint]
    pub struct IBEPairing {
       
    }
}


#[external]
impl IBEPairing {
  
    pub fn pairing(private: Vec<u8>, cu : Vec<u8>) -> Vec<u8>{

        if private.len() != 96 || cu.len() != 48 {
            return Vec::new();
        }

        let mut hash = sha2::Sha256::new();

     
        let _cu_ct = G1Affine::from_compressed(&cu.try_into().unwrap());
        if _cu_ct.is_some().unwrap_u8() == 0 {
            return Vec::new();
        }
        let _cu = _cu_ct.unwrap();

     
        let pr_ct = G2Affine::from_compressed(&private.try_into().unwrap());
        if pr_ct.is_some().unwrap_u8() == 0 {
            return Vec::new();
        }
        let pr = pr_ct.unwrap();

        let r_gid = pairing(&_cu, &pr);

        hash.update(b"IBE-H2");
        hash.update(r_gid.to_bytes().to_vec());

       
        let h_r_git: &[u8] = &hash.finalize().to_vec()[0..32];
        let out: [u8; 32] = match h_r_git.try_into() {
            Ok(val) => val,
            Err(_) => return Vec::new(),
        };

        out.to_vec()

     
}



}

