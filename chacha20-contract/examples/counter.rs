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
 
    let rpc_url ="https://stylus-testnet.arbitrum.io/rpc";
    let program_address = "0xe990a7747090E384CF79Fb669265Dc8fe2F96fB6";
    abigen!(
        Decrypter,
        r#"[
            
        function decryptTx(uint8[] memory tx, uint8[] memory skbytes, address ibe_contract, address decrypter_contract) external view returns (uint8[] memory)

        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = "5b5e3e6144572c7bfb257c6083f0ffaba94b79727411fcd072d3ca8cd2b96d8c";
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));
    let c = [
            97, 103, 101, 45, 101, 110, 99, 114, 121, 112, 116, 105, 111, 110, 46, 111, 114, 103,
            47, 118, 49, 10, 45, 62, 32, 100, 105, 115, 116, 73, 66, 69, 10, 114, 119, 108, 71,
            114, 52, 87, 104, 109, 51, 99, 108, 73, 107, 120, 69, 47, 70, 87, 72, 114, 89, 75, 51,
            68, 76, 88, 118, 65, 109, 107, 54, 98, 97, 75, 118, 54, 54, 86, 104, 77, 73, 88, 101,
            120, 105, 117, 105, 75, 115, 57, 97, 113, 51, 73, 65, 120, 121, 118, 86, 113, 122, 71,
            113, 10, 67, 97, 71, 105, 102, 77, 67, 98, 52, 52, 97, 114, 88, 83, 52, 112, 116, 47,
            90, 82, 55, 107, 99, 53, 119, 67, 116, 56, 117, 83, 78, 80, 112, 51, 106, 81, 43, 84,
            76, 87, 113, 110, 69, 83, 43, 119, 73, 80, 82, 57, 89, 98, 120, 65, 47, 85, 102, 105,
            120, 81, 111, 121, 84, 48, 10, 54, 106, 120, 70, 115, 119, 108, 78, 71, 81, 50, 85, 57,
            111, 89, 117, 112, 47, 83, 57, 112, 81, 10, 45, 45, 45, 32, 82, 71, 75, 80, 115, 55,
            88, 66, 107, 54, 72, 48, 83, 77, 74, 113, 82, 122, 105, 66, 86, 103, 87, 102, 82, 48,
            53, 68, 104, 97, 110, 105, 75, 48, 66, 114, 50, 116, 78, 99, 121, 99, 56, 10, 152, 214,
            234, 209, 59, 136, 118, 65, 151, 122, 93, 188, 167, 183, 26, 167, 161, 112, 12, 1, 100,
            175, 60, 231, 243, 212, 87, 231, 69, 134, 44, 102, 192, 116, 173, 224, 188, 200, 215,
            193, 167, 157, 199, 46, 170, 65, 46, 6, 157, 208, 104, 12, 188, 112, 7, 18, 16, 169,
            92, 172, 126, 78, 40, 149, 215,
        ];
    let pk =[
        132, 219, 183, 104, 17, 129, 230, 157, 183, 26, 153, 233, 66, 115, 68, 164, 71, 138,
        139, 41, 17, 243, 198, 239, 54, 161, 137, 27, 46, 107, 79, 207, 238, 92, 105, 66, 202,
        66, 80, 46, 175, 230, 252, 126, 199, 130, 246, 13,
    ];
    
        let skbytes = [
            180, 94, 231, 64, 60, 139, 63, 77, 251, 219, 173, 163, 74, 124, 6, 10, 129, 139, 151,
            186, 102, 134, 86, 99, 150, 127, 59, 169, 18, 212, 67, 132, 48, 180, 58, 172, 181, 219,
            30, 166, 33, 104, 186, 198, 23, 29, 20, 141, 15, 107, 179, 56, 147, 33, 220, 105, 191,
            20, 32, 206, 3, 203, 206, 179, 228, 207, 247, 100, 37, 47, 155, 29, 212, 118, 240, 159,
            79, 249, 88, 182, 208, 106, 20, 154, 236, 61, 92, 86, 122, 253, 31, 5, 161, 65, 125,
            200,
        ];
        let ibe_contract: Address = "0x891565D05F42946A1c720d041E4DF69D8D490f94".parse()?;
        let decrypter_contract: Address = "0x2F04Fb351a70a450Ac0B4a4593Ec07fF9849d410".parse()?;
        // 0x891565D05F42946A1c720d041E4DF69D8D490f94 // ibe
        // 0x2F04Fb351a70a450Ac0B4a4593Ec07fF9849d410 // decrypter
    let decrypter = Decrypter::new(address, client);
   
    let binding = decrypter.decrypt_tx(c.to_vec(), skbytes.to_vec(), ibe_contract, decrypter_contract);
    let num = binding.call().await?;
    let string = String::from_utf8(num).expect("Invalid UTF-8 sequence");
    println!("value = {:?}", string);
   //println!("Counter number value = {:?}", num);

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
