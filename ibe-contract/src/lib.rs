
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



// impl<'de> Deserialize<'de> for Ciphertext {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         enum Field { U, V, W };

//         impl<'de> Deserialize<'de> for Field {
//             fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
//             where
//                 D: Deserializer<'de>,
//             {
//                 struct FieldVisitor;

//                 impl<'de> Visitor<'de> for FieldVisitor {
//                     type Value = Field;

//                     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                         formatter.write_str("`u`, `v` or `w`")
//                     }

//                     fn visit_str<E>(self, value: &str) -> Result<Field, E>
//                     where
//                         E: de::Error,
//                     {
//                         match value {
//                             "u" => Ok(Field::U),
//                             "v" => Ok(Field::V),
//                             "w" => Ok(Field::W),
//                             _ => Err(de::Error::unknown_field(value, FIELDS)),
//                         }
//                     }
//                 }

//                 deserializer.deserialize_identifier(FieldVisitor)
//             }
//         }

//         struct CiphertextVisitor;

//         impl<'de> Visitor<'de> for CiphertextVisitor {
//             type Value = Ciphertext;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("struct Ciphertext")
//             }

//             fn visit_seq<V>(self, mut seq: V) -> Result<Ciphertext, V::Error>
//             where
//                 V: SeqAccess<'de>,
//             {
//                 let u_compressed: Vec<u8> = seq.next_element()?
//                     .ok_or_else(|| de::Error::invalid_length(0, &self))?;
//                 let u = G1Affine::from_compressed(<&[u8; 48]>::try_from(&u_compressed[..]).unwrap()).unwrap();

//                 let v = seq.next_element()?
//                     .ok_or_else(|| de::Error::invalid_length(1, &self))?;
//                 let w = seq.next_element()?
//                     .ok_or_else(|| de::Error::invalid_length(2, &self))?;

//                 Ok(Ciphertext { u, v, w })
//             }
//         }

//         const FIELDS: &'static [&'static str] = &["u", "v", "w"];
//         deserializer.deserialize_struct("Ciphertext", FIELDS, CiphertextVisitor)
//     }
// }

const BLOCK_SIZE: usize = 32;


/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{prelude::sol_storage, prelude::sol_interface, stylus_proc::{entrypoint, external}};

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
           let r_gid = pairing_contract.pairing(self,private,cu.clone())?;
            // ibepairing.ibePairing(private,cu.try_into());
            // let cu = G1Affine::from_compressed(&c.u.to_compressed()).unwrap();
            // let pr = G2Affine::from_compressed(&private).unwrap();
            // let r_gid = pairing(&cu, &pr);
         
           // panic!("{:?}")
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


