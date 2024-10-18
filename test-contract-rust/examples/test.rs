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
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: program <contract_address> <private_key>");
        std::process::exit(1);
    }

    let contract_address = &args[1];
    let private_key = &args[2];

    let rpc_endpoint = "https://sepolia-rollup.arbitrum.io/rpc";
    
    abigen!(
        AuctionContract,
        r#"[
            function setVars(address decrypter, uint128 deadline, uint128 id, uint128 fee) external
            function submitEncBid(uint8[] memory tx, string calldata condition) external returns (uint8[] memory)
            function submitKey(string calldata condition, uint8[] memory key) external returns (uint8[] memory)
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_endpoint)?;
    let contract_address: Address = contract_address.parse()?;

    let wallet = LocalWallet::from_str(private_key)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let decrypter_address: Address = "0xcb5aadb5bf01d6b685219e98d7c5713b7ac73042".parse()?;

    let auction_contract = AuctionContract::new(contract_address, client);

    let bid_data = [
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

    let secret_key_bytes = [
        180, 94, 231, 64, 60, 139, 63, 77, 251, 219, 173, 163, 74, 124, 6, 10, 129, 139, 151, 186,
        102, 134, 86, 99, 150, 127, 59, 169, 18, 212, 67, 132, 48, 180, 58, 172, 181, 219, 30, 166,
        33, 104, 186, 198, 23, 29, 20, 141, 15, 107, 179, 56, 147, 33, 220, 105, 191, 20, 32, 206,
        3, 203, 206, 179, 228, 207, 247, 100, 37, 47, 155, 29, 212, 118, 240, 159, 79, 249, 88,
        182, 208, 106, 20, 154, 236, 61, 92, 86, 122, 253, 31, 5, 161, 65, 125, 200,
    ];

    let tx = auction_contract.set_vars(decrypter_address, 123456789, 14, 0);
    let _ = tx.send().await?;

    sleep(Duration::from_secs(10));

    let enc_bid_tx = auction_contract.submit_enc_bid(bid_data.to_vec(), String::from_str("Random_IBE_ID").unwrap());
    let enc_bid_result = enc_bid_tx.send().await?;
    println!("Bid submitted = {:?}", enc_bid_result);

    sleep(Duration::from_secs(10));

    let key_submission_tx = auction_contract.submit_key(String::from_str("Random_IBE_ID").unwrap(), secret_key_bytes.to_vec());
    let key_submission_result = key_submission_tx.call().await?;
    let result = String::from_utf8(key_submission_result.clone()).expect("Invalid UTF-8 sequence");
    println!("Highest bid = {:?}", result);

    Ok(())
}
