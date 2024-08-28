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
   
   
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let program_address = arg1.as_str();
   
    abigen!(
        IBE,
        r#"[
      
    
     function decrypt(uint8[] memory private, uint8[] memory cu, address pairing_contract) external view returns (string memory)
    
      
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
    let c2 = hex::decode(arg6).unwrap();
    let skbytes = hex::decode(arg4).unwrap();

    let decrypter_adr: Address = "0x7324de0d624ea2a147c76e3a6f81155558033237".parse()?;
    let pairing_contract_addr: Address = Address::from_str("0xbdb3a69bd70cded40a2cd449779dec4983c3569d").unwrap();
    let decrypter = IBE::new(decrypter_adr, client);
    let binding4 = decrypter.decrypt(skbytes,c,pairing_contract_addr).gas_price(100000000).gas(29000000);
    let num3 = binding4.call().await?;
    let bytes: Vec<u8> = num3.bytes().collect();
    
    // Filter out the null bytes and retain only significant bytes
    let significant_bytes: Vec<u8> = bytes.into_iter().filter(|&b| b != 0).collect();

    println!("{:?}", significant_bytes);


    Ok(())
}


