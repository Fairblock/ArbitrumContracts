use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::eyre;
use ic_bls12_381::{ G1Affine, G2Affine};
use std::env;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use stylus_sdk::call::*;

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
        Hasher,
        r#"[
      
    
     function verify(uint8[] memory sigma, uint8[] memory msg, uint8[] memory cu) external view returns (bool)
    
      
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


    let sigma = vec![19, 65, 37, 162, 246, 189, 27, 29, 191, 32, 125, 222, 21, 26, 134, 64, 201, 126, 119, 89, 233, 172, 171, 145, 25, 190, 114, 209, 165, 61, 138, 5];
    let msg = vec![212, 19, 27, 222, 185, 232, 136, 98, 249, 3, 118, 190, 124, 91, 65, 210, 99, 96, 200, 195, 91, 90, 61, 245, 82, 158, 35, 19, 139, 96, 47, 137];
    let cu = vec![173, 168, 6, 103, 237, 18, 208, 174, 179, 199, 176, 242, 232, 91, 53, 254, 133, 102, 64, 175, 87, 116, 220, 227, 41, 65, 125, 198, 218, 216, 214, 188, 240, 180, 163, 226, 18, 106, 157, 58, 215, 108, 129, 3, 169, 121, 170, 13];
    
 
    let hasher = Hasher::new(address, client);
    let binding = hasher
        .verify(sigma,msg,cu)
        .gas_price(100000000)
        .gas(29000000);

    let out = binding.call().await?;
    
    println!("{:?}",out);
    assert!(out);
    Ok(())
}
