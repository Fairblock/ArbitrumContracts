#!/bin/bash

# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Private key
sk=<PRIVATE_KEY>

# Build the contract
echo -e "${YELLOW}Building the contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null 2>&1

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully.${NC}"
else
    echo -e "${RED}Build failed.${NC}"
    exit 1
fi

# Deploy the contract
echo -e "${YELLOW}Deploying the IBE contract...${NC}"
outputIbe=$(cargo +nightly-2024-05-20-x86_64-unknown-linux-gnu stylus deploy -e https://sepolia-rollup.arbitrum.io/rpc --private-key="$sk" --wasm-file=./target/wasm32-unknown-unknown/release/ibe.wasm 2>/dev/null)

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

echo -e "${GREEN}IBE contract address: $addressIbe${NC}"

# Run the IBE example
echo -e "${YELLOW}Running IBE example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example ibe "$addressIbe" "$sk"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE example ran successfully.${NC}"
else
    echo -e "${RED}IBE example failed.${NC}"
    exit 1
fi
