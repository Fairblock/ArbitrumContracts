
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


use bls12_381_plus::G1Affine;


use sha2::Digest;


use serde::{ Deserialize};

#[derive(Clone, Deserialize)]
pub struct Ciphertext {
    pub u: G1Affine,
    pub v: Vec<u8>,
    pub w: Vec<u8>,
}


const BLOCK_SIZE: usize = 32;


use stylus_sdk::{prelude::sol_storage, prelude::sol_interface, stylus_proc::{entrypoint, external}};

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
        function verify(uint8[] memory sigma, uint8[] memory msg) external pure returns (uint8[] memory);
    }
}

#[external]
impl IBE {
   
    pub fn decrypt(&self,private: Vec<u8>, cv: Vec<u8>, cw: Vec<u8>, cu: Vec<u8>, pairing_contract:IIBEPairing, hasher_contract: IHasher) -> Result<Vec<u8>, stylus_sdk::call::Error>{

       
        assert!(cw.len() <= BLOCK_SIZE, "ciphertext too long for the block size");
    
        // 1. Compute sigma = V XOR H2(e(rP,private))
        let sigma = {
            let mut hash = sha2::Sha256::new();
           let r_gid = pairing_contract.pairing(self,private,cu.clone())?;
            hash.update(b"IBE-H2");
            hash.update(r_gid);
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
        let result_affine = hasher_contract.verify(self,sigma,msg.clone())?;
        let cu_byte: [u8;48] = cu.clone().try_into().unwrap();
        assert_eq!(cu_byte.to_vec(), result_affine);
    
        Ok(msg)
    }
    
   
 

}
fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}


