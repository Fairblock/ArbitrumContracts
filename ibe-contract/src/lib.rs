#![cfg_attr(not(feature = "export-abi"), no_main)]
#![no_std]
extern crate alloc;




use stylus_sdk::function_selector;
use stylus_sdk::prelude::sol_interface;
use core::str::FromStr;
use alloc::{format, vec};
use alloc::vec::Vec;
use stylus_sdk::{alloy_primitives::{hex::ToHexExt, Address}, alloy_sol_types, call::{Call, MethodError}};

use sha2::Digest;


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
    pub fn decrypt(&self, private: Vec<u8>, cv: Vec<u8>, cw: Vec<u8>, cu: Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        if private.len() != 96 || cu.len() != 48 || cv.len() > BLOCK_SIZE || cw.len() > BLOCK_SIZE {
            return Err(stylus_sdk::call::Error::Revert(vec![1]));
        }

        let pairing_contract_addr: Address = Address::from_str("0xc79784b402c4a1681c16e5600df92a32ce062a10").map_err(|_| stylus_sdk::call::Error::Revert(vec![2]))?;
        let hashing_contract_addr: Address = Address::from_str("0x6e50a9114406678ecc3d1731eb666d203e263bf9").map_err(|_| stylus_sdk::call::Error::Revert(vec![3]))?;

        let sigma = {
            let ibe = IIBEPairing { address: pairing_contract_addr };
            let h_r_git = ibe.pairing(Call::new(), private, cu.clone()).map_err(|_| stylus_sdk::call::Error::Revert(vec![4]))?;
            xor(h_r_git.as_slice(), &cv)
        };

        let msg = {
            let mut hash = sha2::Sha256::new();
            hash.update(b"IBE-H4");
            hash.update(&sigma);
            let h_sigma = &hash.finalize()[0..BLOCK_SIZE];
            xor(h_sigma, &cw)
        };

        let hasher = IHasher { address: hashing_contract_addr };
        let verify_res = hasher.verify(Call::new(), sigma.clone(), msg.clone(), cu).map_err(|_| stylus_sdk::call::Error::Revert(vec![5]))?;
        
        if !verify_res {
            return Err(stylus_sdk::call::Error::Revert(vec![6]));
        }

        Ok(msg)
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}
