
#![cfg_attr(not(feature = "export-abi"), no_main)]
#![no_std]
extern crate alloc;



use ethabi::Token;

use ic_bls12_381::{pairing, G2Affine};
use stylus_sdk::alloy_sol_types::SolValue;
use stylus_sdk::call::{call, RawCall};
use stylus_sdk::function_selector;
use stylus_sdk::prelude::sol_interface;
use core::str::FromStr;
use alloc::{format, vec};
use alloc::vec::Vec;

use stylus_sdk::{alloy_primitives::{hex::ToHexExt, Address}, alloy_sol_types, call::{Call, MethodError}};

use ic_bls12_381::G1Affine;

// use bls12_381_plus::G1Affine as G1Affine2;
use sha2::Digest;
use alloc::string::{String, ToString};

use serde::{ Deserialize};

// #[derive(Clone, Deserialize)]
// pub struct Ciphertext {
//     pub u: G1Affine2,
//     pub v: Vec<u8>,
//     pub w: Vec<u8>,
// }


const BLOCK_SIZE: usize = 32;


use stylus_sdk::{prelude::sol_storage, stylus_proc::{entrypoint, external}};

sol_storage! {
    #[entrypoint]
    pub struct IBE {
       
    }
}

sol_interface! {
    interface IIBEPairing {
        function pairing(uint8[] memory private, uint8[] memory cu) external view returns (uint8[] memory);
       
    }
    interface IHasher {
        function verify(uint8[] memory sigma, uint8[] memory msg, uint8[] memory cu) external view returns (bool);
    }

}
#[external]
impl IBE {
   
    pub fn decrypt(&self,private: Vec<u8>, cv: Vec<u8>, cw: Vec<u8>, cu: Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error>{
     
        let pairing_contract_addr: Address = Address::from_str(&"0xaa5fcbafbf1b25e92b73124216b8bfc823f61b1e").unwrap();
        let hashing_contract_addr: Address = Address::from_str(&"0x9ff55191d27f5e311549687527dabb44096253c5").unwrap();

    ////////////////////////////////////////
    assert!(cw.len() <= BLOCK_SIZE, "ciphertext too long for the block size");
    
    // 1. Compute sigma = V XOR H2(e(rP,private))
    let sigma = {
    
        let ibe = IIBEPairing{address:pairing_contract_addr};
       let h_r_git = ibe.pairing(Call::new(),private,cu.clone()).unwrap();
        xor(h_r_git.as_slice(), &cv)
    };

    // 2. Compute Msg = W XOR H4(sigma)
    let msg = {
        let mut hash = sha2::Sha256::new();
        hash.update(b"IBE-H4");
        hash.update(&sigma);
        let h_sigma = &hash.finalize().to_vec()[0..BLOCK_SIZE];
        xor(h_sigma, &cw)
    };

    // 3. Check U = G^r
    let hasher = IHasher{address:hashing_contract_addr};
    let verify_res = hasher.verify(Call::new(), sigma, msg.clone(),cu).unwrap();
    assert!(verify_res);

     Ok(msg)
   
    }
    
   
 

}
fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}


