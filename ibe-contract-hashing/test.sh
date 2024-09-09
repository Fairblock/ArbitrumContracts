# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

outputIbehashing=$(cargo  +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

addressIbehashing=$(echo "$outputIbehashing" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

echo "ibe-contract-hashing address: $addressIbehashing" 

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example hashing  "$addressIbehashing" "$sk"
