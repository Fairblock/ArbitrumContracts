use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::env;
use std::str::FromStr;
use std::sync::Arc;




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
      
    
     function decrypter(uint8[] memory key, uint8[] memory nonce, uint8[] memory ciphertext) external view returns (uint8[] memory)
    
      
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
    let key: Vec<u8> = vec![
        212, 19, 27, 222, 185, 232, 136, 98, 249, 3, 118, 190, 124, 91, 65, 210, 99, 96, 200, 195,
        91, 90, 61, 245, 82, 158, 35, 19, 139, 96, 47, 137,
    ];
    let nonce = vec![
        37, 61, 47, 1, 244, 206, 42, 96, 20, 9, 7, 125, 207, 71, 69, 210,
    ];
    let ciphertext = vec![
        104, 143, 189, 62, 0, 194, 29, 184, 189, 149, 107, 25, 206, 151, 8, 95, 30, 144, 61,
        203, 218, 96, 122, 237, 116, 192, 86,
    ];
    let decrypter_chacha_20 = DecrypterChacha20::new(address, client);
    let binding = decrypter_chacha_20
        .decrypter(key.try_into().unwrap(), nonce.try_into().unwrap(), ciphertext);

    let out = binding.call().await?;
    let result = String::from_utf8(out).unwrap();
    println!("Output: {:?} - Expected output: \"Hello World\"", result);
    assert_eq!(result, "Hello World");
    Ok(())
}
