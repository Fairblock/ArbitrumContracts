# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af
# ########## first time use only for deploying the required contracts
# cd ../ibe-contract-hashing

# cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

# outputIbeHashing=$(cargo  +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc --private-key="$sk" --wasm-file=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

# addressIbeHashing=$(echo "$outputIbeHashing" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract-hashing address: $addressIbeHashing" 

# ######## first time use only for deploying the required contracts
cd ../ibe-contract-pairing

cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null

outputIbepairing=$(cargo  +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

addressIbepairing=$(echo "$outputIbepairing" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

echo "ibe-contract-pairing address: $addressIbepairing" 

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

# # cd ../custom-contract
# cargo clean
# cargo update
# cargo build  --target=wasm32-unknown-unknown --release > /dev/null

# outputcustom=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file=./target/wasm32-unknown-unknown/release/custom.wasm)

# addresscustom=$(echo "$outputcustom" | grep "deployed code at address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

# echo "custom-contract address: $addresscustom"

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

# cd ../../ShareGenerator

# output=$(./ShareGenerator generate 1 1 | jq '.')

# # Extract the 'Value' and 'MasterPublicKey' fields
# value=$(echo "$output" | jq -r '.Shares[0].Value')
# master_public_key=$(echo "$output" | jq -r '.MasterPublicKey')
# echo "pk: $master_public_key"
# output=$(./ShareGenerator derive "$value" 0 14-56 | jq '.')

# key_share=$(echo "$output" | jq -r '.KeyShare')
# echo "key share : $key_share"
# cd ../encrypter

# cipher=$(./encrypter 14-56 "$master_public_key" "171")
# echo "cipher: $cipher"
# cipher2=$(./encrypter 14-56 "$master_public_key" "67")
# echo "cipher: $cipher2"

# #######

# cd ./custom-test
# cipher=6167652d656e6372797074696f6e2e6f72672f76310a2d3e20646973744942450a73386d427877414c4e58644c66337a62627771545a51744142366d3373433439794743362b327767796c425147657932383445524f524c465154474b725838350a346a465539566757414f325852693050665332792f476b373465386837534936732f364d5673723072576a6775614d6463505039796e364976746d44416e2b730a6233553565387633494c34764a433952316c366732410a2d2d2d206368686d6a33464233634f794753673965483068704d75517261624f6255767765375865504338377034450af6e4089bb51f822d0631a0b34d61472e617b48fa178d9956e15df4d66a2ed5f9e82eb430822d01e120e965
# key_share=b45ee7403c8b3f4dfbdbada34a7c060a818b97ba66865663967f3ba912d4438430b43aacb5db1ea62168bac6171d148d0f6bb3389321dc69bf1420ce03cbceb3e4cff764252f9b1dd476f09f4ff958b6d06a149aec3d5c567afd1f05a1417dc8
# addressDc=0x2235d678529d918441f97c1d59869f0885609e43
# addresscustom=0x2235d678529d918441f97c1d59869f0885609e43
# cipher2=6167652d656e6372797074696f6e2e6f72672f76310a2d3e20646973744942450a73386d427877414c4e58644c66337a62627771545a51744142366d3373433439794743362b327767796c425147657932383445524f524c465154474b725838350a346a465539566757414f325852693050665332792f476b373465386837534936732f364d5673723072576a6775614d6463505039796e364976746d44416e2b730a6233553565387633494c34764a433952316c366732410a2d2d2d206368686d6a33464233634f794753673965483068704d75517261624f6255767765375865504338377034450af6e4089bb51f822d0631a0b34d61472e617b48fa178d9956e15df4d66a2ed5f9e82eb430822d01e120e965
# addressregistry=0x2235d678529d918441f97c1d59869f0885609e43
# RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example testnet --target=x86_64-unknown-linux-gnu "$addresscustom" "$addressDec" "$cipher" "$key_share" "$sk" "$cipher2" "$addressregistry"

# # ######## for testing with fairyring and client

# # cd ../../../fairyring

# # echo 1 | ./start-fairy.sh > fairylog.txt & cd ../fairybridge

# # ########
# # sleep 10

# # cargo run --target x86_64-unknown-linux-gnu