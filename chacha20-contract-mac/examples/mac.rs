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
        MacChacha20,
        r#"[ 
     function headermac(uint8[] memory key, uint8[] memory body) external view returns (uint8[] memory)
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
    let body: Vec<u8> = vec![
        173, 168, 6, 103, 237, 18, 208, 174, 179, 199, 176, 242, 232, 91, 53, 254, 133, 102, 64,
        175, 87, 116, 220, 227, 41, 65, 125, 198, 218, 216, 214, 188, 240, 180, 163, 226, 18, 106,
        157, 58, 215, 108, 129, 3, 169, 121, 170, 13, 234, 127, 4, 159, 177, 247, 59, 204, 90, 152,
        203, 160, 131, 136, 223, 36, 211, 185, 122, 213, 31, 223, 2, 151, 90, 8, 122, 40, 179, 138,
        248, 166, 30, 19, 1, 80, 73, 15, 191, 118, 254, 56, 244, 233, 225, 163, 134, 242, 170, 53,
        157, 182, 234, 233, 250, 207, 221, 64, 151, 102, 93, 207, 188, 132,
    ];
    let mac: Vec<u8> = vec![
        175, 132, 247, 0, 18, 122, 58, 12, 4, 139, 182, 77, 14, 83, 87, 41, 78, 209, 199, 48, 159,
        183, 136, 131, 38, 12, 38, 148, 120, 13, 41, 215,
    ];
    let mac_contract = MacChacha20::new(address, client);
    let binding = mac_contract
        .headermac(key, body);

    let out = binding.call().await?;
    println!("output mac: {:?} - expected mac: {:?}", out, mac);
    assert_eq!(mac, out);
    Ok(())
}
