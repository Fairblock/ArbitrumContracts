//! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::eyre;
use stylus_sdk::call::*;
use std::env;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use sha2::{Sha256, Digest};
use alloy_primitives::hex_literal::hex;

fn sha256_of_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Ensure there are enough arguments
    if args.len() < 7 {
        eprintln!(
            "Usage: program program_address decrypter cipher sk wallet_key cipher2"
        );
        std::process::exit(1);
    }

    let arg1 = &args[1];
    let arg2 = &args[2];
    let arg3 = &args[3];
    let arg4 = &args[4];
    let arg5 = &args[5];
    let arg6 = &args[6];
    let arg7 = &args[7];
   
   
    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";
    let program_address = arg1.as_str();
   
    abigen!(
        Auction,
        r#"[
      
        function setVars(address registry, address decrypter, uint128 deadline, uint128 id, uint128 fee) external

        function checkCondition() external returns (string memory)
    
        function submitEncBid(uint8[] memory tx, string calldata condition, string calldata hash) external payable returns (uint8[] memory)
    
        function submitKey(string calldata k) external returns (bool)

        function decrypt(uint8[] memory key, uint128 i) external returns (bool)

        function dec(uint8[] memory tx, uint8[] memory key) external returns (uint8[] memory)
    
        function checkWinner() external returns (string memory)

        function submitDecrypted(string[] memory data) external returns (bool)

        function checkFinished() external returns (bool)
        ]"#
       
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = arg5.as_str();
  
    
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));
    
    let c = hex::decode(arg3).unwrap();
    let plain = arg6;
    let skbytes = hex::decode(arg4).unwrap();

    let registry_contract: Address = arg7.parse()?;
  
    let decrypter: Address = arg2.parse()?;

    let custom = Auction::new(address, client);

    let binding = custom.set_vars(
        registry_contract,
        decrypter,
        456,
        1,
        0,
    );
    let _ = binding.send().await?;

   
    thread::sleep(Duration::from_secs(10));
    let hash = sha256_of_string(plain);
    let binding2 = custom.submit_enc_bid(c.to_vec(), String::from_str("14-56").unwrap(),hash);
    let _ = binding2.send().await?;
    
   

    thread::sleep(Duration::from_secs(20));
    let binding4 = custom.decrypt(skbytes.to_vec(), 0).gas_price(100000000).gas(40000000);
    let num3 = binding4.send().await?;
    println!("decrypt on chain : {:?}", num3);

    
    let mut data  : Vec<String> = vec![]; 
  
    data.insert(0, plain.to_string());
        

    thread::sleep(Duration::from_secs(10));
    let binding6 = custom.submit_decrypted(data).gas_price(100000000).gas(40000000);
    let num5 = binding6.send().await?;
    println!("off chain : {:?}", num5);
    Ok(())


}


