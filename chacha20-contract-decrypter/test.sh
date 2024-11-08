#!/bin/bash


# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Get the absolute path of the directory where the script is located
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to the script directory (where `Cargo.toml` is located)
cd "$script_dir"

# Source the .env file using the absolute path
source "$script_dir/../.env"

# Private key
# SECRET_KEY=<PRIVATE_KEY>
# RPC url
rpc_url=https://sepolia-rollup.arbitrum.io/rpc

# Build the contract
echo -e "${YELLOW}Building the DecrypterChacha20 contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null 2>&1

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully.${NC}"
else
    echo -e "${RED}Build failed.${NC}"
    exit 1
fi

# Deploy the contract
echo -e "${YELLOW}Deploying the DecrypterChacha20 contract...${NC}"
outputdecrypter=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$SECRET_KEY" --wasm-file=./target/wasm32-unknown-unknown/release/chacha20.wasm  2>/dev/null)

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressdecrypter=$(echo "$outputdecrypter" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressdecrypter" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

echo -e "${GREEN}CHACHA20_DECRYPTER_CONTRACT_ADDRESS: $addressdecrypter${NC}"

# Run the example
echo -e "${YELLOW}Running example with the DecrypterChacha20 contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example decrypter "$addressdecrypter" "$SECRET_KEY"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}DecrypterChacha20 example ran successfully.${NC}"
else
    echo -e "${RED}DecrypterChacha20 example failed.${NC}"
    exit 1
fi
