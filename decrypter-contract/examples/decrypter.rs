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
        Decrypter,
        r#"[
      
    
     function decrypt(uint8[] memory c, uint8[] memory skbytes) external view returns (uint8[] memory)
    
      
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
    let c: Vec<u8> = vec![
        97, 103, 101, 45, 101, 110, 99, 114, 121, 112, 116, 105, 111, 110, 46, 111, 114, 103, 47,
        118, 49, 10, 45, 62, 32, 100, 105, 115, 116, 73, 66, 69, 10, 114, 97, 103, 71, 90, 43, 48,
        83, 48, 75, 54, 122, 120, 55, 68, 121, 54, 70, 115, 49, 47, 111, 86, 109, 81, 75, 57, 88,
        100, 78, 122, 106, 75, 85, 70, 57, 120, 116, 114, 89, 49, 114, 122, 119, 116, 75, 80, 105,
        69, 109, 113, 100, 79, 116, 100, 115, 103, 81, 79, 112, 101, 97, 111, 78, 10, 54, 110, 56,
        69, 110, 55, 72, 51, 79, 56, 120, 97, 109, 77, 117, 103, 103, 52, 106, 102, 74, 78, 79, 53,
        101, 116, 85, 102, 51, 119, 75, 88, 87, 103, 104, 54, 75, 76, 79, 75, 43, 75, 89, 101, 69,
        119, 70, 81, 83, 81, 43, 47, 100, 118, 52, 52, 57, 79, 110, 104, 111, 52, 98, 121, 10, 113,
        106, 87, 100, 116, 117, 114, 112, 43, 115, 47, 100, 81, 74, 100, 109, 88, 99, 43, 56, 104,
        65, 10, 45, 45, 45, 32, 114, 52, 84, 51, 65, 66, 74, 54, 79, 103, 119, 69, 105, 55, 90, 78,
        68, 108, 78, 88, 75, 85, 55, 82, 120, 122, 67, 102, 116, 52, 105, 68, 74, 103, 119, 109,
        108, 72, 103, 78, 75, 100, 99, 10, 37, 61, 47, 1, 244, 206, 42, 96, 20, 9, 7, 125, 207, 71,
        69, 210, 104, 143, 189, 62, 0, 194, 29, 184, 189, 149, 107, 25, 206, 151, 8, 95, 30, 144,
        61, 203, 218, 96, 122, 237, 116, 192, 86,
    ];

    let sk: Vec<u8> = vec![
        180, 94, 231, 64, 60, 139, 63, 77, 251, 219, 173, 163, 74, 124, 6, 10, 129, 139, 151, 186,
        102, 134, 86, 99, 150, 127, 59, 169, 18, 212, 67, 132, 48, 180, 58, 172, 181, 219, 30, 166,
        33, 104, 186, 198, 23, 29, 20, 141, 15, 107, 179, 56, 147, 33, 220, 105, 191, 20, 32, 206,
        3, 203, 206, 179, 228, 207, 247, 100, 37, 47, 155, 29, 212, 118, 240, 159, 79, 249, 88,
        182, 208, 106, 20, 154, 236, 61, 92, 86, 122, 253, 31, 5, 161, 65, 125, 200,
    ];
    let decrypter = Decrypter::new(address, client);
    let binding = decrypter.decrypt(c, sk).gas_price(100000000).gas(29000000);

    let out = binding.call().await?;
    let result = String::from_utf8(out).unwrap();
    println!("{:?}", result);
    assert_eq!(result, "Hello World");
    Ok(())
}
