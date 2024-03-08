
#![cfg_attr(not(feature = "export-abi"), no_main)]

extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use sha2::Digest;


use ic_bls12_381::{Scalar,G1Projective,G1Affine};

use num_bigint::{BigInt, Sign};

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{prelude::sol_storage, stylus_proc::{entrypoint, external}};


sol_storage! {
    #[entrypoint]
    pub struct Hasher {
       
    }
}


#[external]

impl Hasher {
   pub fn verify (sigma: Vec<u8>, msg: Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error> {
           let r_g = {
           
            
            let r = h3(sigma,msg)?;
            let rs = Scalar::from_bytes(&r).unwrap();
            let g1_base_projective = G1Projective::from(G1Affine::generator());
            (g1_base_projective * rs)
        };
       let result_affine = G1Affine::from(r_g);
       
      Ok(result_affine.to_compressed().to_vec())
   }
 

}

pub  fn h3(sigma: Vec<u8>, msg: Vec<u8>) -> Result<[u8;32], stylus_sdk::call::Error> {
        
    let mut hasher = sha2::Sha256::new();

    // Hashing H3Tag, sigma and msg
    hasher.update(b"IBE-H3");
    hasher.update(sigma);
    hasher.update(msg);
    let buffer = hasher.finalize_reset();

    // Create a BigInt for hashable
    let mut hashable = BigInt::new(Sign::Plus, Vec::new());
    let canonical_bit_len = (hashable.bits() + 7) / 8 * 8;
    let actual_bit_len = hashable.bits();
    let to_mask = canonical_bit_len - actual_bit_len;

    for i in 1..65535u16 {
        let iter = i.to_le_bytes();
        hasher.update(&iter);
        hasher.update(&buffer);
        let mut hashed = hasher.finalize_reset().to_vec();

        // Applying masking
        if hashable.to_bytes_be().1[0] & 0x80 != 0 {
            hashed[0] >>= to_mask;
        } else {
            let l =hashed.len();
            hashed[l - 1] >>= to_mask;
        }
        
        hashed[0] = hashed[0]/2;
        
        hashed.reverse();
        // Unmarshal and check if within the modulo
       let v = BigInt::from_bytes_le(Sign::Plus, &hashed); 
       let vec = v.to_bytes_le().1;
       let array: &[u8; 32] = vec.get(..32)
    .and_then(|slice| slice.try_into().ok())
    .expect("Vec is shorter than 32 bytes");
        let sc = Scalar::from_bytes(&array.clone());
        
        if sc.is_some().into(){
         
            return Ok(*array);
        }
          
    }
let my_error = stylus_sdk::call::Error::Revert(vec![0]);
Err(my_error)
   
}