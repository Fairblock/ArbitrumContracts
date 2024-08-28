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
        IBE,
        r#"[
      
    
     function decrypt(uint8[] private,uint8[] cv,uint8[] cw,uint8[] cu, string memory pairing_contract, string memory hasher_contract) external view returns (uint8[] memory)
    
      
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
        226, 49, 84, 245, 88, 22, 0, 237, 151, 70, 45, 15, 125, 45, 178, 252, 105, 59, 225, 239,
        33, 237, 34, 58, 179, 254, 140, 86, 202, 244, 173, 104,
    ];
    let cw: Vec<u8> = vec![
        224, 185, 163, 29, 112, 243, 253, 202, 126, 136, 190, 217, 131, 2, 127, 172, 111, 117, 57,
        123, 203, 247, 32, 190, 47, 36, 47, 81, 214, 94, 160, 216,
    ];
    let cu: Vec<u8> = vec![
        179, 201, 129, 199, 0, 11, 53, 119, 75, 127, 124, 219, 111, 10, 147, 101, 11, 64, 7, 169,
        183, 176, 46, 61, 200, 96, 186, 251, 108, 32, 202, 80, 80, 25, 236, 182, 243, 129, 17, 57,
        18, 197, 65, 49, 138, 173, 127, 57,
    ];
    let pairing_contract_addr: String = "0x13544f0d527f74706b862ae87f2b13b89ee1d190".to_string();
    let hasher_contract_addr: String = "0x2c397820261d13080e404b4ff25aa0e16e2062b2".to_string();
    let ibe = IBE::new(address, client);
    let binding = ibe
        .decrypt(sig, cv, cw, cu, pairing_contract_addr, hasher_contract_addr)
        .gas_price(100000000)
        .gas(29000000);

    let out = binding.call().await?;
    println!("{:?}", out);

    Ok(())
}
