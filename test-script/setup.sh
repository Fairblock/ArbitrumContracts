read -p "Enter the wallet private key: " sk

# #######

# cd ../ibe-contract

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputIbe=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/ibe.wasm)

# addressIbe=$(echo "$outputIbe" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract address: $addressIbe" 

# ########

# cd ../chacha20-contract-decrypter

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputChachadec=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/chacha20.wasm)

# addressChachadec=$(echo "$outputChachadec" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "chacha20-contract-decrypter address: $addressChachadec"

# ########

# cd ../chacha20-contract-mac

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputChachamac=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/chacha20mac.wasm)

# addressChachamac=$(echo "$outputChachamac" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "chacha20-contract-mac address: $addressChachamac"

# #######

# cd ../decrypter-contract

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputDec=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/decrypter.wasm)

# addressDec=$(echo "$outputDec" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "decrypter-contract address: $addressDec"

#######

cd ../custom-contract

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

outputcustom=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/custom.wasm)

addresscustom=$(echo "$outputcustom" | grep "Deploying program to address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

echo "custom-contract address: $addresscustom"

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

cipher=$(./encrypter 1456 "$master_public_key" "125")
echo "cipher: $cipher"
cipher2=$(./encrypter 1456 "$master_public_key" "67")
echo "cipher: $cipher2"

#######
#addresscustom=0x07B7452db98B09724287ad4780c23F8e802D6Fe8
addressIbe=0xE9d3Ad58d2d697B08B2ce777541Ddf30F1f060EC
addressChachadec=0x438cc3c7E2Da22D897Ac8b5dc9509628B67EA13f
addressChachamac=0x73c90f1B5c1DE9c73e4c68E6e1D4Ad7E48C5a7Fc
addressDec=0x16651b15030968B8D1485F605aE6F4293a3D332E
cd ../ArbitrumContracts/test-script/custom-test

RUST_BACKTRACE=1 cargo run --example counter --target=x86_64-unknown-linux-gnu "$addresscustom" "$addressIbe" "$addressChachadec" "$addressChachamac" "$addressDec" "$cipher" "$key_share" "$sk" "$cipher2"