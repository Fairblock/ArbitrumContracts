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



#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Ensure there are enough arguments
    if args.len() < 10 {
        eprintln!(
            "Usage: program program_address ibe_contract decrypter_contract mac_contract decrypter cipher sk wallet_key cipher2"
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
    let arg8 = &args[8];
    let arg9 = &args[9];
    let arg10 = &args[10];
   
    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";
    let program_address = arg1.as_str();
   
    abigen!(
        Auction,
        r#"[
      
        function setVars(address registry, address decrypter, address ibe_contract, address decrypter_contract, address mac_contract, uint128 deadline, uint128 id, uint128 fee) external

            function submitEncBid(uint8[] memory tx, string calldata condition) external returns (uint8[] memory)
        
            function submitKey(string calldata k) external returns (bool)
        
   

            function checkWinner() external returns (string memory)

            function checkFinished() external returns (bool)

            function dec(uint8[] memory tx, uint8[] memory key, address ibe_c, address dec_c, address mac_c) external returns (uint8[] memory)
        ]"#
       
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = arg8.as_str();
  
    
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));
    
    let c = hex::decode(arg6).unwrap();
    let c2 = hex::decode(arg9).unwrap();
    let skbytes = hex::decode(arg7).unwrap();

    let registry_contract: Address = arg10.parse()?;

    let ibe_contract: Address = arg2.parse()?;
   
    let decrypter_contract: Address = arg3.parse()?;
   
    let mac_contract: Address = arg4.parse()?;
  
    let decrypter: Address = arg5.parse()?;

    let custom = Auction::new(address, client);

    let binding = custom.set_vars(
        registry_contract,
        decrypter,
        ibe_contract,
        decrypter_contract,
        mac_contract,
        456,
        1,
        0,
    );
    let _ = binding.send().await?;

   thread::sleep(Duration::from_secs(10));
    let binding2 = custom.submit_enc_bid(c.to_vec(), String::from_str("1456").unwrap());
    let num = binding2.send().await?;
    println!("tx = {:?}", num);
    
    // thread::sleep(Duration::from_secs(20));

    // let binding3 = custom.submit_enc_bid(c2.to_vec(), String::from_str("1456").unwrap());
    // let num2 = binding3.send().await?;
    // println!("tx = {:?}", num2);



 

    // ***** for standalone testing *****

    // thread::sleep(Duration::from_secs(20));
    // let binding4 = custom.submit_key(arg7.to_string());
    // let num3 = binding4.send().await?;
    // println!("highest bid = {:?}", num3);

    //thread::sleep(Duration::from_secs(20));
    let binding4 = custom.dec(c.to_vec(),skbytes.to_vec(),ibe_contract,decrypter_contract,mac_contract).gas_price(100000000).gas(26000000);
    let num3 = binding4.send().await?;
    println!("{:?}", num3);


    Ok(())
}


