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
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

/// Your private key file path.
const ENV_PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";

/// Stylus RPC endpoint url.
const ENV_RPC_URL: &str = "RPC_URL";

/// Deployed pragram address.
const ENV_PROGRAM_ADDRESS: &str = "STYLUS_PROGRAM_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";
    let program_address = "0x2Ce88343d8Df6614DEa8574E43E48A5641e10750";
    abigen!(
        Auction,
        r#"[
      
            function setVars(address fairblock, address decrypter, address ibe_contract, address decrypter_contract, address mac_contract, uint128 total) external

            function submitEncBid(uint8[] memory tx, string calldata condition) external returns (uint8[] memory)
        
            function submitKey(string calldata condition, uint8[] memory key) external returns (uint8[] memory)
        
            function dec(uint8[] memory tx, uint8[] memory key, address ibe_c, address dec_c, address mac_c) external returns (uint8[] memory)
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = "0f5a0ac08cb519614ec16d57c81f5265a814ae73a83bcbc38a975911c40bd252";
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));
    let c = [
        97, 103, 101, 45, 101, 110, 99, 114, 121, 112, 116, 105, 111, 110, 46, 111, 114, 103, 47,
        118, 49, 10, 45, 62, 32, 100, 105, 115, 116, 73, 66, 69, 10, 106, 119, 117, 102, 81, 101,
        115, 53, 75, 71, 69, 75, 104, 67, 104, 109, 88, 79, 101, 49, 102, 65, 43, 56, 107, 57, 109,
        52, 54, 71, 113, 83, 76, 111, 108, 98, 48, 74, 67, 113, 83, 75, 116, 82, 120, 72, 113, 105,
        50, 107, 51, 70, 108, 76, 114, 101, 107, 114, 90, 106, 81, 52, 97, 117, 10, 103, 86, 111,
        99, 113, 66, 106, 90, 101, 109, 105, 82, 66, 54, 86, 79, 83, 70, 54, 110, 74, 113, 117, 43,
        84, 104, 115, 117, 81, 67, 86, 117, 103, 72, 76, 86, 120, 48, 100, 90, 98, 70, 78, 56, 48,
        84, 52, 53, 66, 108, 77, 101, 43, 122, 57, 85, 90, 50, 97, 111, 115, 110, 106, 71, 10, 104,
        53, 111, 79, 67, 57, 51, 84, 90, 98, 69, 53, 79, 79, 97, 83, 85, 112, 111, 43, 69, 81, 10,
        45, 45, 45, 32, 105, 98, 67, 76, 115, 81, 47, 86, 101, 53, 52, 116, 80, 116, 106, 99, 49,
        85, 88, 88, 98, 75, 69, 53, 84, 90, 104, 56, 113, 100, 102, 89, 105, 77, 57, 107, 53, 53,
        70, 100, 107, 108, 107, 10, 216, 219, 176, 112, 82, 16, 95, 62, 198, 100, 66, 28, 145, 63,
        103, 141, 49, 246, 71, 167, 195, 230, 38, 195, 96, 226, 12, 13, 21, 49, 85, 119, 205, 78,
        198,
    ];
    let pk = [
        132, 219, 183, 104, 17, 129, 230, 157, 183, 26, 153, 233, 66, 115, 68, 164, 71, 138, 139,
        41, 17, 243, 198, 239, 54, 161, 137, 27, 46, 107, 79, 207, 238, 92, 105, 66, 202, 66, 80,
        46, 175, 230, 252, 126, 199, 130, 246, 13,
    ];

    let skbytes = [
        180, 94, 231, 64, 60, 139, 63, 77, 251, 219, 173, 163, 74, 124, 6, 10, 129, 139, 151, 186,
        102, 134, 86, 99, 150, 127, 59, 169, 18, 212, 67, 132, 48, 180, 58, 172, 181, 219, 30, 166,
        33, 104, 186, 198, 23, 29, 20, 141, 15, 107, 179, 56, 147, 33, 220, 105, 191, 20, 32, 206,
        3, 203, 206, 179, 228, 207, 247, 100, 37, 47, 155, 29, 212, 118, 240, 159, 79, 249, 88,
        182, 208, 106, 20, 154, 236, 61, 92, 86, 122, 253, 31, 5, 161, 65, 125, 200,
    ];
    let ibe_contract: Address = "0x891565D05F42946A1c720d041E4DF69D8D490f94".parse()?;
    let decrypter_contract: Address = "0x2F04Fb351a70a450Ac0B4a4593Ec07fF9849d410".parse()?;
    let mac_contract: Address = "0x047f15524c8cAbBb636F2a295222dc54224Ec37a".parse()?;
    let decrypter: Address = "0x5E5A1a7725A9FAA081FE6faABC036e9a5244D1F9".parse()?;
   
    let custom = Auction::new(address, client);

    let binding = custom.set_vars(
        ibe_contract,
        decrypter,
        ibe_contract,
        decrypter_contract,
        mac_contract,
        1,
    );
    let _ = binding.send().await?;

     let binding2 = custom.submit_enc_bid(c.to_vec(), String::from_str("Random_IBE_ID").unwrap());
       let num = binding2.send().await?;
     println!("highest bid = {:?}", num);

    let binding3 = custom.submit_key(String::from_str("Random_IBE_ID").unwrap(), skbytes.to_vec());
    let num2 = binding3.call().await?;
    let string2 = String::from_utf8(num2.clone()).expect("Invalid UTF-8 sequence");
    println!("highest bid = {:?}", string2);




    // let num = counter.number().call().await;
    // println!("New counter number value = {:?}", num);
    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
