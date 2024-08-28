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
        MacChacha20,
        r#"[
      
    
     function header_mac(uint8[] memory file_key, string memory type_, string[] memory args, uint8[] memory body) external view returns (uint8[] memory)
    
      
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
    let body: Vec<u8> = vec![179, 201, 129, 199, 0, 11, 53, 119, 75, 127, 124, 219, 111, 10, 147, 101, 11, 64, 7, 169, 183, 176, 46, 61, 200, 96, 186, 251, 108, 32, 202, 80, 80, 25, 236, 182, 243, 129, 17, 57, 18, 197, 65, 49, 138, 173, 127, 57, 226, 49, 84, 245, 88, 22, 0, 237, 151, 70, 45, 15, 125, 45, 178, 252, 105, 59, 225, 239, 33, 237, 34, 58, 179, 254, 140, 86, 202, 244, 173, 104, 224, 185, 163, 29, 112, 243, 253, 202, 126, 136, 190, 217, 131, 2, 127, 172, 111, 117, 57, 123, 203, 247, 32, 190, 47, 36, 47, 81, 214, 94, 160, 216];

    let mac_contract = MacChacha20::new(address, client);
    let binding = mac_contract
        .header_mac(file_key,"distIBE".to_string(),vec![],body)
        .gas_price(100000000)
        .gas(29000000);

    let out = binding.call().await?;
    println!("{:?}", out);

    Ok(())
}
