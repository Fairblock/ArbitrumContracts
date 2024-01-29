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
   
    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";
    let program_address = arg1.as_str();
    //"0x2Ce88343d8Df6614DEa8574E43E48A5641e10750";
    abigen!(
        Auction,
        r#"[
      
        function setVars(address registry, address decrypter, address ibe_contract, address decrypter_contract, address mac_contract, uint128 deadline, uint128 id, uint128 fee) external

            function submitEncBid(uint8[] memory tx, string calldata condition) external returns (uint8[] memory)
        
            function submitKey(string calldata condition, uint8[] memory key) external returns (uint8[] memory)
        
            function dec(uint8[] memory tx, uint8[] memory key, address ibe_c, address dec_c, address mac_c) external returns (uint8[] memory)
        ]"#
       
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = arg8.as_str();
  
    //"0f5dbd99b4fb1a300ca068668f41178bed1062376c4c30a5e7957cfa27258323";
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));
    
    let c = hex::decode(arg6).unwrap();
    let c2 = hex::decode(arg9).unwrap();
    let skbytes = hex::decode(arg7).unwrap();
    let registry_contract: Address = "0x49589dE2cd3b1f91966dDFC4aB7c45fADeEd72A5".parse()?;
    let ibe_contract: Address = arg2.parse()?;
    //"0x891565D05F42946A1c720d041E4DF69D8D490f94".parse()?;
    let decrypter_contract: Address = arg3.parse()?;
    //"0x2F04Fb351a70a450Ac0B4a4593Ec07fF9849d410".parse()?;
    let mac_contract: Address = arg4.parse()?;
    //"0x047f15524c8cAbBb636F2a295222dc54224Ec37a".parse()?;
    let decrypter: Address = arg5.parse()?;
    //"0x5E5A1a7725A9FAA081FE6faABC036e9a5244D1F9".parse()?;
    // 0x891565D05F42946A1c720d041E4DF69D8D490f94 // ibe
    // 0x2F04Fb351a70a450Ac0B4a4593Ec07fF9849d410 // decrypter
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

//    thread::sleep(Duration::from_secs(10));

//     let binding2 = custom.submit_enc_bid(c.to_vec(), String::from_str("1456").unwrap());
//     let num = binding2.send().await?;
//     println!("tx = {:?}", num);

//     thread::sleep(Duration::from_secs(20));

//     let binding3 = custom.submit_enc_bid(c2.to_vec(), String::from_str("1456").unwrap());
//     let num2 = binding3.send().await?;
//     println!("tx = {:?}", num2);

   


    // ***** for standalone testing *****

    //thread::sleep(Duration::from_secs(20));
    // let binding4 = custom.submit_key(String::from_str("1456").unwrap(), skbytes.to_vec());
    // let num3 = binding4.call().await?;
    // let string3 = String::from_utf8(num3.clone()).expect("Invalid UTF-8 sequence");
    // println!("highest bid = {:?}", string3);


    Ok(())
}


