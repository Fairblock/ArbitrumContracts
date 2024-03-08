# read -p "Enter the wallet private key: " sk
sk=c1f754ce022376d07cf5af49d4432212f07fa802f204960cd76bda5ac12ef8ea
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

cd ../decrypter-contract

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

outputDec=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/decrypter.wasm)

addressDec=$(echo "$outputDec" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

echo "decrypter-contract address: $addressDec"

#######

cd ../hash-auction-contract

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

outputcustom=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/custom.wasm)

addresscustom=$(echo "$outputcustom" | grep "Deploying program to address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

echo "hash-auction-contract address: $addresscustom"

#######

# cd ../registry-contract

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputregistry=$(cargo-stylus stylus deploy   --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/registry.wasm)

# addressregistry=$(echo "$outputregistry" | grep "Deploying program to address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

# echo "registry-contract address: $addressregistry"

# CONFIG_FILE="../../fairybridge/config.toml"

# sed -i "s/registry_address = \".*\"/registry_address = \"$addressregistry\"/" $CONFIG_FILE
# sed -i "s/arbitrum_key = \".*\"/arbitrum_key = \"$sk\"/" $CONFIG_FILE
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
#addresscustom=0xbA0683A185f456AD7AeCb16fBF1b5835515ef598
addressregistry=0xb0aC23b333e8F50dD6cb7ABBb46Ed67c3543FAD8
# addressIbe=0xE9d3Ad58d2d697B08B2ce777541Ddf30F1f060EC
# addressChachadec=0x438cc3c7E2Da22D897Ac8b5dc9509628B67EA13f
# addressChachamac=0x73c90f1B5c1DE9c73e4c68E6e1D4Ad7E48C5a7Fc
#addressDec=0x36811CBAb450261d5DABCFc12d6Ef691bdf1D8C4

cd ../ArbitrumContracts/test-script/custom-test

RUST_BACKTRACE=1 cargo run --example datalength --target=x86_64-unknown-linux-gnu "$addresscustom" "$addressDec" "$cipher" "$key_share" "$sk" "$cipher2" "$addressregistry"

########

# cd ../../../fairyring

# echo 1 | ./start-fairy.sh > fairylog.txt & cd ../fairybridge

# ########
# sleep 10

# cargo run --target x86_64-unknown-linux-gnu