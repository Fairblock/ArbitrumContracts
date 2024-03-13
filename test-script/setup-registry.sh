#read -p "Enter the wallet private key: " sk
sk=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659

echo -e "\033[1m\033[43m*******************building and deploying the registry contract...***************************\033[0m"
cd ../registry-contract

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config "profile.release.opt-level='z'" --release > temp.txt 2>&1

outputregistry=$(cargo-stylus stylus deploy -e http://localhost:8547  --private-key="$sk"  --wasm-file-path=./target/wasm32-unknown-unknown/release/r.wasm)

addressregistry=$(echo "$outputregistry" | grep "Deploying program to address" | awk '{print $5}'| sed 's/\x1b\[[0-9;]*m//g')

echo "registry-contract address: $addressregistry"

CONFIG_FILE="../../fairybridge/config.toml"

sed -i "s/registry_address = \".*\"/registry_address = \"$addressregistry\"/" $CONFIG_FILE
sed -i "s/arbitrum_key = \".*\"/arbitrum_key = \"$sk\"/" $CONFIG_FILE
