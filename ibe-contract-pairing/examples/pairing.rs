use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::eyre;
use ic_bls12_381::{pairing, G1Affine, G2Affine};
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
        IBEPairing,
        r#"[
      
    
     function pairing(uint8[] memory private, uint8[] memory cu) external view returns (uint8[579] memory)
    
      
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

    let cu = G1Affine::generator();
    let key = G2Affine::generator();

    let pair = pairing(&cu, &key);
    // println!("{:?}", pair.to_bytes().to_vec());
    println!("{:?}", pair.to_bytes().to_vec());
    let pairing_contract = IBEPairing::new(address, client);
    let binding = pairing_contract
        .pairing(key.to_compressed().to_vec(), cu.to_compressed().to_vec())
        .gas_price(100000000)
        .gas(29000000);

    let out = binding.call().await?;
    // let p= out[64..64+out[63] as usize].to_vec();

    Ok(())
}
