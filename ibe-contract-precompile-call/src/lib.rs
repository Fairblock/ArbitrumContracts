
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



use bls12_381_plus::G1Affine;
//use ic_bls12_381::{G1Affine};
// use num_bigint::{BigInt, Sign};



use sha2::Digest;


use serde::{ Deserialize};

#[derive(Clone, Deserialize)]
pub struct Ciphertext {
    pub u: G1Affine,
    pub v: Vec<u8>,
    pub w: Vec<u8>,
}

extern crate ethabi;
extern crate serde_json;

use ethabi::{ Token};
use std::str::FromStr;



const BLOCK_SIZE: usize = 32;


/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::Address,  call::RawCall, prelude::{sol_interface, sol_storage}, stylus_proc::{entrypoint, external}};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
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
/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl IBE {
   
    pub fn decrypt(&self,private: Vec<u8>, cv: Vec<u8>, cw: Vec<u8>, cu: Vec<u8>, pairing_contract:IIBEPairing, hasher_contract: IHasher) -> Result<Vec<u8>, stylus_sdk::call::Error>{

        // let c : Ciphertext = serde_json::from_slice(cipher.as_slice()).unwrap();
        
        assert!(cw.len() <= BLOCK_SIZE, "ciphertext too long for the block size");
    
        // 1. Compute sigma = V XOR H2(e(rP,private))
        let sigma = {
            let mut hash = sha2::Sha256::new();
            //todo
            // let r_gid = RawCall::new_static()
            // .call(
            //     Address::with_last_byte(84 as u8),
            //     &[private.clone(), cu.clone()].concat(),
            // ).unwrap();
            let input = encode_function(private, cu.clone());
      
            let r_gid = RawCall::new_static()
                .call(
                    Address::from_str("0x0000000000000000000000000000000000000084").unwrap(),
                    &input,
                )
                .unwrap();
         
            let gid= r_gid[64..].to_vec();
          // let r_gid = pairing_contract.pairing(self,private,cu.clone())?;
            // ibepairing.ibePairing(private,cu.try_into());
            // let cu = G1Affine::from_compressed(&c.u.to_compressed()).unwrap();
            // let pr = G2Affine::from_compressed(&private).unwrap();
            // let r_gid = pairing(&cu, &pr);
         
           // panic!("{:?}")
            hash.update(b"IBE-H2");
            hash.update(gid);
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
        //todo
        let result_affine = hasher_contract.verify(self,sigma,msg.clone())?;
        let cu_byte: [u8;48] = cu.clone().try_into().unwrap();
        assert_eq!(cu_byte.to_vec(), result_affine);
    
        Ok(msg)
    }
    
   
 

}
fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}


fn encode_function(private: Vec<u8>,cu: Vec<u8> )-> Vec<u8>{
    let function_signature: [u8; 4] = [0x64, 0x79, 0x6d, 0x57];
    // Prepare the inputs as Tokens
    let inputs: Vec<Token> = vec![Token::Bytes(private), Token::Bytes(cu)];

    // Encode the inputs with the function signature
    let mut encoded = function_signature.to_vec();
    let encoded_params = ethabi::encode(&inputs);

    // Combine the function signature with the encoded parameters
    encoded.extend_from_slice(&encoded_params);

    encoded
}