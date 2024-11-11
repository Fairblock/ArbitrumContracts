#!/bin/bash

# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the absolute path of the directory where the script is located
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to the script directory (where `Cargo.toml` is located)
cd "$script_dir"

# Source the .env file using the absolute path
source "$script_dir/../.env"

# Build the contract
echo -e "${YELLOW}Building the contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release 

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully.${NC}"
else
    echo -e "${RED}Build failed.${NC}"
    exit 1
fi

# Deploy the contract
echo -e "${YELLOW}Deploying the IBE hashing contract...${NC}"
outputIbehashing=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$PRIVATE_KEY_1" --wasm-file=./target/wasm32-unknown-unknown/release/ibe-contract-hashing.wasm)

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE hashing contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressIbehashing=$(echo "$outputIbehashing" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressIbehashing" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

# Print contract address with unique marker
echo "IBE_HASHING_CONTRACT_ADDRESS $addressIbehashing"

# Run the hashing example
echo -e "${YELLOW}Running hashing example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example hashing "$addressIbehashing" "$PRIVATE_KEY_1"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Hashing example ran successfully.${NC}"
else
    echo -e "${RED}Hashing example failed.${NC}"
    exit 1
fi
