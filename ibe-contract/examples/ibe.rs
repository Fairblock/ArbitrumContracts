use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
    utils::hex::ToHexExt,
};

use eyre::{eyre, Ok};
use ic_bls12_381::{pairing, G1Affine, G1Projective, G2Affine, Scalar};
use num_bigint::{BigInt, Sign};
use std::env;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
const BLOCK_SIZE: usize = 32;
use sha2::Digest;
use stylus_sdk::{call::*, function_selector};
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
      
    
     function decrypt(uint8[] memory private,uint8[] memory cv,uint8[] memory cw,uint8[] memory cu) external view returns (uint8[] memory)
 
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
    let sig: Vec<u8> = vec![
        180, 94, 231, 64, 60, 139, 63, 77, 251, 219, 173, 163, 74, 124, 6, 10, 129, 139, 151, 186,
        102, 134, 86, 99, 150, 127, 59, 169, 18, 212, 67, 132, 48, 180, 58, 172, 181, 219, 30, 166,
        33, 104, 186, 198, 23, 29, 20, 141, 15, 107, 179, 56, 147, 33, 220, 105, 191, 20, 32, 206,
        3, 203, 206, 179, 228, 207, 247, 100, 37, 47, 155, 29, 212, 118, 240, 159, 79, 249, 88,
        182, 208, 106, 20, 154, 236, 61, 92, 86, 122, 253, 31, 5, 161, 65, 125, 200,
    ];
    let cv: Vec<u8> = vec![
        234, 127, 4, 159, 177, 247, 59, 204, 90, 152, 203, 160, 131, 136, 223, 36, 211, 185, 122,
        213, 31, 223, 2, 151, 90, 8, 122, 40, 179, 138, 248, 166,
    ];
    let cw: Vec<u8> = vec![
        30, 19, 1, 80, 73, 15, 191, 118, 254, 56, 244, 233, 225, 163, 134, 242, 170, 53, 157, 182,
        234, 233, 250, 207, 221, 64, 151, 102, 93, 207, 188, 132,
    ];
    let cu: Vec<u8> = vec![
        173, 168, 6, 103, 237, 18, 208, 174, 179, 199, 176, 242, 232, 91, 53, 254, 133, 102, 64,
        175, 87, 116, 220, 227, 41, 65, 125, 198, 218, 216, 214, 188, 240, 180, 163, 226, 18, 106,
        157, 58, 215, 108, 129, 3, 169, 121, 170, 13,
    ];
    let _cu = G1Affine::from_compressed(&cu.clone().clone().try_into().unwrap()).unwrap();
    let pr = G2Affine::from_compressed(&sig.clone().try_into().unwrap()).unwrap();
    let pair = pairing(&_cu, &pr);

    let sigma = {
        let mut hash = sha2::Sha256::new();
        let r_gid = pair.to_bytes().to_vec();
        hash.update(b"IBE-H2");
        hash.update(r_gid.clone());
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

    let ibe = IBE::new(address, client);
    let binding = ibe
        .decrypt(sig, cv, cw, cu)
        .gas_price(100000000)
        .gas(29000000);
    let out = binding.call().await?;
    assert_eq!(out, msg);
    println!("output: {:?} - msg : {:?}", out, msg);
    Ok(())
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn h3(sigma: Vec<u8>, msg: Vec<u8>) -> [u8; 32] {
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
            let l = hashed.len();
            hashed[l - 1] >>= to_mask;
        }

        hashed[0] = hashed[0] / 2;

        hashed.reverse();
        // Unmarshal and check if within the modulo
        let v = BigInt::from_bytes_le(Sign::Plus, &hashed);
        let vec = v.to_bytes_le().1;
        let array: &[u8; 32] = vec
            .get(..32)
            .and_then(|slice| slice.try_into().ok())
            .expect("Vec is shorter than 32 bytes");
        let sc = Scalar::from_bytes(&array.clone());

        if sc.is_some().into() {
            return *array;
        }
    }
    let my_error = stylus_sdk::call::Error::Revert(vec![0]);
    [0u8; 32]
}
