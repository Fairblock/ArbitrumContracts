# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af
# ########## first time use only for deploying the required contracts
# cd ../ibe-contract-hashing

# cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

# outputIbeHashing=$(cargo  +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc --private-key="$sk" --wasm-file=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

# addressIbeHashing=$(echo "$outputIbeHashing" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract-hashing address: $addressIbeHashing" 

# ######## first time use only for deploying the required contracts
# cd ../ibe-contract-pairing

# cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

# outputIbepairing=$(cargo  +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

# addressIbepairing=$(echo "$outputIbepairing" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract-pairing address: $addressIbepairing" 

# # ###### first time use only for deploying the required contracts

# cd ../ibe-contract

# cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

# outputIbe=$(cargo  +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc  --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/ibe.wasm)

# addressIbe=$(echo "$outputIbe" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract address: $addressIbe" 

# ######## first time use only for deploying the required contracts

# cd ../chacha20-contract-decrypter

# cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

# outputChachadec=$(cargo  +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc  --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/chacha20.wasm)

# addressChachadec=$(echo "$outputChachadec" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "chacha20-contract-decrypter address: $addressChachadec"

######## first time use only for deploying the required contracts

# cd ../chacha20-contract-mac

# cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

# outputChachamac=$(cargo  +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc  --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/chacha20mac.wasm)

# addressChachamac=$(echo "$outputChachamac" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "chacha20-contract-mac address: $addressChachamac"

# #######

# cd ../decrypter-contract

# cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

# outputDec=$(cargo +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc  --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/decrypter.wasm)

# addressDec=$(echo "$outputDec" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "decrypter-contract address: $addressDec"

# #######

cd ../test-contract-rust


cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

outputcustom=$(cargo +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc  --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/custom.wasm)


addresscustom=$(echo "$outputcustom" | grep "deployed code at address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

echo "custom-contract address: $addresscustom"

# #######

# cd ../registry-contract
# cargo clean
# cargo update
# cargo build  --target=wasm32-unknown-unknown --release > /dev/null

# outputregistry=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/registry.wasm)

# addressregistry=$(echo "$outputregistry" | grep "deployed code at address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

# echo "registry-contract address: $addressregistry"

# CONFIG_FILE="../../fairybridge/config.toml"

# sed -i "s/registry_address = \".*\"/registry_address = \"$addressregistry\"/" $CONFIG_FILE
# sed -i "s/arbitrum_key = \".*\"/arbitrum_key = \"$sk\"/" $CONFIG_FILE
# #######

# sleep 5

cd ../test-script/ShareGenerator

output=$(./ShareGenerator generate 1 1 | jq '.')

# Extract the 'Value' and 'MasterPublicKey' fields
value=$(echo "$output" | jq -r '.Shares[0].Value')
master_public_key=$(echo "$output" | jq -r '.MasterPublicKey')
echo "pk: $master_public_key"
output=$(./ShareGenerator derive "$value" 0 14-56 | jq '.')

key_share=$(echo "$output" | jq -r '.KeyShare')
echo "key share : $key_share"
cd ../encrypter

cipher=$(./encrypter 14-56 "$master_public_key" "171")
echo "cipher: $cipher"
cipher2=$(./encrypter 14-56 "$master_public_key" "67")
echo "cipher: $cipher2"

# #######

cd ../custom-test
addressDec=0x54410c800e89f30d46a745f6f6809967fe6d049d

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example testnet --target=x86_64-unknown-linux-gnu "$addresscustom" "$addressDec" "$cipher" "$key_share" "$sk" "$cipher2"

# # ######## for testing with fairyring and client

# # cd ../../../fairyring

# # echo 1 | ./start-fairy.sh > fairylog.txt & cd ../fairybridge

# # ########
# # sleep 10

# # cargo run --target x86_64-unknown-linux-gnu