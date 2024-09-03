use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address, utils::hex::ToHexExt,
};

use num_bigint::{BigInt, Sign};
use eyre::{eyre, Ok};
use ic_bls12_381::{pairing, G1Affine, G1Projective, G2Affine, Scalar};
use std::env;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
const BLOCK_SIZE: usize = 32;
use stylus_sdk::{call::*, function_selector};
use sha2::Digest;
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Ensure there are enough arguments
    if args.len() < 2 {
        eprintln!("Usage: program program_address wallet_key");
        std::process::exit(1);
    }

    let arg1 = &args[1];
    let arg2 = &args[2];

    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let program_address = arg1.as_str();

    abigen!(
        IBE,
        r#"[
      
    
     function decrypt(uint8[96] memory private,uint8[] memory cv,uint8[] memory cw,uint8[48] memory cu, string memory pairing_contract, string memory hasher_contract) external view returns (uint8[] memory)
 
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let wallet_key = arg2.as_str();

    let wallet = LocalWallet::from_str(&wallet_key)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));
    let sig: Vec<u8> = vec![147, 224, 43, 96, 82, 113, 159, 96, 125, 172, 211, 160, 136, 39, 
    79, 101, 89, 107, 208, 208, 153, 32, 182, 26, 181, 218, 97, 187, 220, 127, 80, 73, 51, 76, 241, 18, 19, 148, 93, 87, 229, 172, 125, 5, 93, 4, 43, 126, 2, 74, 162, 178, 240, 143, 10, 145, 38, 8, 5, 39, 45, 197, 16, 81, 198, 228, 122, 212, 250, 64, 59, 2, 180, 81, 11, 100, 122, 227, 209, 119, 11, 172, 3, 38, 168, 5, 187, 239, 212, 128, 86, 200, 193, 33, 189, 184];
    let cv: Vec<u8> = vec![
        226, 49, 84, 245, 88, 22, 0, 237, 151, 70, 45, 15, 125, 45, 178, 252, 105, 59, 225, 239,
        33, 237, 34, 58, 179, 254, 140, 86, 202, 244, 173, 104,
    ];
    let cw: Vec<u8> = vec![
        224, 185, 163, 29, 112, 243, 253, 202, 126, 136, 190, 217, 131, 2, 127, 172, 111, 117, 57,
        123, 203, 247, 32, 190, 47, 36, 47, 81, 214, 94, 160, 216,
    ];
    let cu: Vec<u8> = vec![151, 241, 211, 167, 49, 151, 215, 148, 38, 149, 99, 140, 79, 169, 172, 15, 195, 104, 140, 79, 151, 116, 185, 5, 161, 78, 58, 63, 23, 27, 172, 88, 108, 85, 232
    , 63, 249, 122, 26, 239, 251, 58, 240, 10, 219, 34, 198, 187];
    let _cu = G1Affine::from_compressed(&cu.clone().clone().try_into().unwrap()).unwrap();
    let pr = G2Affine::from_compressed(&sig.clone().try_into().unwrap()).unwrap();
    let pair = pairing(&_cu, &pr);
   
    let sigma = {
        let mut hash = sha2::Sha256::new();

       let r_gid=pair.to_bytes().to_vec();
        
        hash.update(b"IBE-H2");
        hash.update(r_gid.clone());
        println!("{:?}",r_gid);
        let h_r_git: &[u8] = &hash.finalize().to_vec()[0..BLOCK_SIZE];
        println!("{:?} {}", h_r_git, h_r_git.len());
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
    let r_g = {
           
            
        let r = h3(sigma.to_vec(),msg.to_vec());
        let rs = Scalar::from_bytes(&r).unwrap();
        let g1_base_projective = G1Projective::from(G1Affine::generator());
        (g1_base_projective * rs)
    };
   let result_affine = G1Affine::from(r_g);
   

   println!("msg : {:?} sigma: {:?} a: {:?}", msg, sigma, result_affine.to_compressed());
    let pairing_contract_addr: String = "0x6c81613befc3271dfd835a35eb79aac372409b88".to_string();
    let hasher_contract_addr: String = "0x6c81613befc3271dfd835a35eb79aac372409b88".to_string();
    let ibe = IBE::new(address, client);
    // let binding = ibe
    //     .decrypt(sig, cv, cw, cu, pairing_contract_addr, hasher_contract_addr)
    //     .gas_price(100000000)
    //     .gas(29000000);
    // let _cu = G1Affine::from_compressed(&cu.clone().clone().try_into().unwrap()).unwrap();
    // let pr = G2Affine::from_compressed(&sig.clone().try_into().unwrap()).unwrap();
    // let pair = pairing(&_cu, &pr);
    // println!("{:?}", pair.to_bytes().to_vec());
  
   let binding = ibe.decrypt(sig.try_into().unwrap(),cv,cw,cu.try_into().unwrap(),pairing_contract_addr,hasher_contract_addr).gas_price(100000000).gas(29000000);
    let out = binding.call().await?;
 
    println!("{:?}", out);
    Ok(())
}
fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

pub  fn h3(sigma: Vec<u8>, msg: Vec<u8>) -> [u8;32] {
        
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
         
            return *array;
        }
          
    }
let my_error = stylus_sdk::call::Error::Revert(vec![0]);
[0u8;32]
   
}