#!/bin/bash

# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

source "../.env"

# RPC url
rpc_url=https://sepolia-rollup.arbitrum.io/rpc

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
echo -e "${YELLOW}Deploying the test contract...${NC}"
outputTest=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$PRIVATE_KEY_1" --wasm-file=./target/wasm32-unknown-unknown/release/custom.wasm)
echo $outputTest
if [ $? -eq 0 ]; then
    echo -e "${GREEN}Test contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressTest=$(echo "$outputTest" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressTest" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

echo -e "${GREEN}Test contract address: $addressTest${NC}"

# Run the Test example
echo -e "${YELLOW}Running Test example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example test "$addressTest" "$PRIVATE_KEY_1"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Test example ran successfully.${NC}"
else
    echo -e "${RED}Test example failed.${NC}"
    exit 1
fi
