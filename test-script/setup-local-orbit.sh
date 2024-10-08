read -p "Enter the registry contract address: " addressregistry
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af
export RUST_LOG=info
# ####### first time use only for deploying the required contracts

# cd ../ibe-contract-hashing

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputIbeHashing=$(cargo-stylus stylus deploy -e http://localhost:8547 --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

# addressIbeHashing=$(echo "$outputIbeHashing" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract-hashing address: $addressIbeHashing" 

####### first time use only for deploying the required contracts

# cd ../ibe-contract-precompile-call

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputIbe=$(cargo-stylus stylus deploy -e http://localhost:8547  --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/ibe.wasm)

# addressIbe=$(echo "$outputIbe" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "ibe-contract address: $addressIbe" 

# ######## first time use only for deploying the required contracts

# cd ../chacha20-contract-decrypter

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputChachadec=$(cargo-stylus stylus deploy -e http://localhost:8547  --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/chacha20.wasm)

# addressChachadec=$(echo "$outputChachadec" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "chacha20-contract-decrypter address: $addressChachadec"

# ######## first time use only for deploying the required contracts

# cd ../chacha20-contract-mac

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputChachamac=$(cargo-stylus stylus deploy  -e http://localhost:8547 --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/chacha20mac.wasm)

# addressChachamac=$(echo "$outputChachamac" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "chacha20-contract-mac address: $addressChachamac"

# #######

# cd ../decrypter-contract

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > /dev/null

# outputDec=$(cargo-stylus stylus deploy -e http://localhost:8547  --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/decrypter.wasm)

# addressDec=$(echo "$outputDec" | grep "Deploying program to address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')

# echo "decrypter-contract address: $addressDec"

#######
echo -e "\033[1m\033[43m*******************building and deploying the custom contract...***************************\033[0m"

cd ../custom-contract-precompile

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > temp.txt 2>&1

outputcustom=$(cargo-stylus stylus deploy -e http://localhost:8449 --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/custom.wasm)

addresscustom=$(echo "$outputcustom" | grep "Deploying program to address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

echo -e "\033[1m\033[43m*******************custom-contract address: $addresscustom"
#addresscustom=0x9cB4e942FFb375659d30d3287d77227AD4607393
#######
# echo "building and deploying the registry contract...***************************\033[0m"
# cd ../registry-contract

# cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > temp.txt

# outputregistry=$(cargo-stylus stylus deploy -e http://localhost:8449  --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/r.wasm)

# addressregistry=$(echo "$outputregistry" | grep "Deploying program to address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

# echo "registry-contract address: $addressregistry"

# CONFIG_FILE="../../fairybridge/config.toml"

# sed -i "s/registry_address = \".*\"/registry_address = \"$addressregistry\"/" $CONFIG_FILE
# sed -i "s/arbitrum_key = \".*\"/arbitrum_key = \"$sk\"/" $CONFIG_FILE
######
echo -e "\033[1m\033[43m*******************generating the ciphertexts...***************************\033[0m"
sleep 5

cd ../../ShareGenerator

output=$(./ShareGenerator generate 1 1 | jq '.')

# Extract the 'Value' and 'MasterPublicKey' fields
value=$(echo "$output" | jq -r '.Shares[0].Value')
master_public_key=$(echo "$output" | jq -r '.MasterPublicKey')
#master_public_key=aac3880d4978a79f8c01630927e4b06020e45c6b7ded016a7df99d35a000fec24590d610a1c119d9a18d6b4270aad4f1
echo "pk: $master_public_key"
output=$(./ShareGenerator derive "$value" 0 14-56 | jq '.')

key_share=$(echo "$output" | jq -r '.KeyShare')
#key_share=aac729475df68c8f499d6d92bff23f8f7b91d817a73b5b1bbc84d1d07b07bcc2df3dd7a2363290a92858e0a1d5e5db671371acfe2c1862855f646014fb5a258da3a9161906bb9dad3848e08e9bdac6528d5166327f81dc762c1e739f04ffe4cd
echo "key share : $key_share"
cd ../encrypter

cipher=$(./encrypter 14-56 "$master_public_key" "111")
echo "cipher: $cipher"
cipher2=$(./encrypter 14-56 "$master_public_key" "2345")
echo "cipher: $cipher2"

#######
echo -e "\033[1m\033[43m*******************running the test...***************************\033[0m"
#addresscustom=0xCfadF54A4a5739Eb251Fc1d1c04D56203b4DdE09
addressDec=0x0000000000000000000000000000000000000094
cd ../ArbitrumContracts/test-script/custom-test

RUST_BACKTRACE=1 cargo run --example local-orbit --target=x86_64-unknown-linux-gnu "$addresscustom" "$addressDec" "$cipher" "$key_share" "$sk" "$cipher2" "$addressregistry" 

# ######## for testing with fairyring and client

# cd ../../../fairyring

# echo 1 | ./start-fairy.sh > fairylog.txt & cd ../fairybridge

# # # ########
# sleep 10

# cargo run --target x86_64-unknown-linux-gnu