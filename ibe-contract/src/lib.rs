
#![cfg_attr(not(feature = "export-abi"), no_main)]
#![no_std]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
use ethabi::Token;
use core::str::FromStr;
use alloc::{format, vec};
use alloc::vec::Vec;

use stylus_sdk::{alloy_primitives::{hex::ToHexExt, Address}, alloy_sol_types, call::{call, MethodError}};

use bls12_381_plus::G1Affine;
use stylus_sdk::{ call::Call};

use sha2::Digest;
use alloc::string::String;

use serde::{ Deserialize};

#[derive(Clone, Deserialize)]
pub struct Ciphertext {
    pub u: G1Affine,
    pub v: Vec<u8>,
    pub w: Vec<u8>,
}


const BLOCK_SIZE: usize = 32;


use stylus_sdk::{prelude::sol_storage, stylus_proc::{entrypoint, external}};

sol_storage! {
    #[entrypoint]
    pub struct IBE {
       
    }
}


#[external]
impl IBE {
    pub fn test(private: Vec<u8>, cu: Vec<u8>)-> Result<Vec<u8>, stylus_sdk::call::Error>{
        let pairing_contract_addr: Address = Address::from_str("0x95c3a60e6b338bdca03f35ea2408c60e4c9a13b7").unwrap();
     
        let calldata = encode_function(private, cu.clone(),[0x08,0xe7,0x98,0xa4]);
      
        match call(Call::new(), pairing_contract_addr, &calldata) {
        
            Ok(out) => {
             
                return Ok(out);
            },
          
            Err(e) =>{ let error_string = format!("Contract call failed with error: {:?}", e);
                
                 return Ok(error_string.into_bytes().to_vec()); },
        }
    }
    pub fn decrypt(&self,private: Vec<u8>, cv: Vec<u8>, cw: Vec<u8>, cu: Vec<u8>, pairing_contract:String, hasher_contract: String) -> Result<Vec<u8>, stylus_sdk::call::Error>{
     
        let pairing_contract_addr: Address = Address::from_str(&pairing_contract).unwrap();
        let hashing_contract_addr: Address = Address::from_str(&hasher_contract).unwrap();

    ////////////////////////////////////////
    assert!(cw.len() <= BLOCK_SIZE, "ciphertext too long for the block size");
    
    // 1. Compute sigma = V XOR H2(e(rP,private))
    let sigma = {
        let mut hash = sha2::Sha256::new();

        let calldata = encode_function(private, cu.clone(),[0x08,0xe7,0x98,0xa4]);
        let r_gid = match call(Call::new(), pairing_contract_addr, &calldata){
            Ok(value) => value, 
            Err(e) => {
                return Err(e); 
            }
        };
        let r_gid_data= r_gid[64..64+r_gid[63] as usize].to_vec();
        hash.update(b"IBE-H2");
        hash.update(r_gid_data);
        let h_r_git: &[u8] = &hash.finalize().to_vec()[0..BLOCK_SIZE];
        xor(h_r_git, &cv)
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
    let calldata = encode_function(sigma, msg.clone(),[0xfc,0x73,0x5e,0x99]);
    let result_affine = match call(Call::new(), hashing_contract_addr, &calldata){
        Ok(value) => value, 
        Err(e) => {
            return Err(e); 
        }
    };
    let result_affine_data= result_affine[64..64+result_affine[63] as usize].to_vec();
    let cu_byte: [u8;48] = cu.clone().try_into().unwrap();
    assert_eq!(cu_byte.to_vec(), result_affine_data);

    Ok(msg)
   
    }
    
   
 

}
fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}


fn encode_function(input1: Vec<u8>,input2: Vec<u8> , function_signature: [u8;4])-> Vec<u8>{
    // Prepare the inputs as Tokens
    let inputs: Vec<Token> = vec![Token::Bytes(input1), Token::Bytes(input2)];

    // Encode the inputs with the function signature
    let mut encoded = function_signature.to_vec();
    let encoded_params = ethabi::encode(&inputs);

    // Combine the function signature with the encoded parameters
    encoded.extend_from_slice(&encoded_params);

    encoded
}

