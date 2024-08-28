use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::eyre;

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
        DecrypterChacha20,
        r#"[
      
    
     function decrypter(uint8[] memory file_key, uint8[] memory nonce, uint8[] memory s) external view returns (uint8[] memory)
    
      
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
    let file_key: Vec<u8> = vec![135, 23, 37, 192, 118, 19, 149, 225, 252, 248, 146, 59, 33, 59, 145, 74, 119, 232, 232, 73, 40, 248, 139, 52, 22, 38, 17, 215, 44, 251, 134, 124];
    let nonce = vec![];
    let s = vec![0];
    let decrypter_chacha_20 = DecrypterChacha20::new(address, client);
    let binding = decrypter_chacha_20
        .decrypter(file_key,nonce,s)
        .gas_price(100000000)
        .gas(29000000);

    let out = binding.call().await?;
    println!("{:?}", out);

    Ok(())
}
