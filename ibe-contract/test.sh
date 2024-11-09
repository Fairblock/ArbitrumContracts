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

# Accept the address of IBE_HASHING for initialization
IBE_HASHING=$1

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
echo -e "${YELLOW}Deploying the IBE contract...${NC}"
outputIbe=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$SECRET_KEY" --wasm-file=./target/wasm32-unknown-unknown/release/ibe.wasm)

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressIbe=$(echo "$outputIbe" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressIbe" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

echo -e "${GREEN}IBE_CONTRACT_ADDRESS: $addressIbe${NC}"

# Initialize the ibe contract with the ibe IBE_HASHING contract address
echo -e "${YELLOW}Initializing the IBE contract...${NC}"
initialize_output=$(cast send $addressIbe "initialize(string)" $IBE_HASHING --rpc-url $rpc_url --private-key $SECRET_KEY 2>&1)

if [[ $? -ne 0 ]] || [[ $initialize_output == *"revert"* ]]; then
    echo -e "${RED}Initialization failed or reverted. Output:${NC}"
    echo "$initialize_output"
    exit 1
fi

echo -e "${GREEN}Initialization completed successfully.${NC}"

# Run the IBE example
echo -e "${YELLOW}Running IBE example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example ibe "$addressIbe" "$SECRET_KEY"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE example ran successfully.${NC}"
else
    echo -e "${RED}IBE example failed.${NC}"
    exit 1
fi


