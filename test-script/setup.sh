read -p "Enter the wallet private key: " sk
########## first time use only for deploying the required contracts
# cd ../ibe-contract-hashing

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputIbeHashing=$(cargo-stylus stylus deploy --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

# addressIbeHashing=$(echo "$outputIbeHashing" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract-hashing address: $addressIbeHashing" 

# ######## first time use only for deploying the required contracts
# cd ../ibe-contract-pairing

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputIbepairing=$(cargo-stylus stylus deploy --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

# addressIbepairing=$(echo "$outputIbepairing" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract-pairing address: $addressIbepairing" 

# ####### first time use only for deploying the required contracts

# cd ../ibe-contract

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputIbe=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/ibe.wasm)

# addressIbe=$(echo "$outputIbe" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract address: $addressIbe" 

# ######## first time use only for deploying the required contracts

# cd ../chacha20-contract-decrypter

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputChachadec=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/chacha20.wasm)

# addressChachadec=$(echo "$outputChachadec" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "chacha20-contract-decrypter address: $addressChachadec"

# ######## first time use only for deploying the required contracts

# cd ../chacha20-contract-mac

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputChachamac=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/chacha20mac.wasm)

# addressChachamac=$(echo "$outputChachamac" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "chacha20-contract-mac address: $addressChachamac"

# #######

cd ../decrypter-contract

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

outputDec=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/decrypter.wasm)

addressDec=$(echo "$outputDec" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

echo "decrypter-contract address: $addressDec"

#######

cd ../custom-contract

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

outputcustom=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/custom.wasm)

addresscustom=$(echo "$outputcustom" | grep "Deploying program to address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

echo "custom-contract address: $addresscustom"

#######

cd ../registry-contract

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

outputregistry=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/registry.wasm)

addressregistry=$(echo "$outputregistry" | grep "Deploying program to address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

echo "registry-contract address: $addressregistry"

CONFIG_FILE="../../fairybridge/config.toml"

sed -i "s/registry_address = \".*\"/registry_address = \"$addressregistry\"/" $CONFIG_FILE
sed -i "s/arbitrum_key = \".*\"/arbitrum_key = \"$sk\"/" $CONFIG_FILE
#######

sleep 5

cd ../../ShareGenerator

output=$(./ShareGenerator generate 1 1 | jq '.')

# Extract the 'Value' and 'MasterPublicKey' fields
value=$(echo "$output" | jq -r '.Shares[0].Value')
master_public_key=$(echo "$output" | jq -r '.MasterPublicKey')
echo "pk: $master_public_key"
output=$(./ShareGenerator derive "$value" 0 1456 | jq '.')

key_share=$(echo "$output" | jq -r '.KeyShare')
echo "key share : $key_share"
cd ../encrypter

cipher=$(./encrypter 1456 "$master_public_key" "171")
echo "cipher: $cipher"
cipher2=$(./encrypter 1456 "$master_public_key" "67")
echo "cipher: $cipher2"

#######

cd ../ArbitrumContracts/test-script/custom-test

RUST_BACKTRACE=1 cargo run --example testnet --target=x86_64-unknown-linux-gnu "$addresscustom" "$addressDec" "$cipher" "$key_share" "$sk" "$cipher2" "$addressregistry"

# ######## for testing with fairyring and client

# cd ../../../fairyring

# echo 1 | ./start-fairy.sh > fairylog.txt & cd ../fairybridge

# ########
# sleep 10

# cargo run --target x86_64-unknown-linux-gnu