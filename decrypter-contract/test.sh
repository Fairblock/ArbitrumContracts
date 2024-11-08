#!/bin/bash

# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

source "../.env"

# Private key
# SECRET_KEY=<PRIVATE_KEY>
# RPC url
rpc_url=https://sepolia-rollup.arbitrum.io/rpc
# Helper contracts addresses
# Ibe=0x24f7d1544e572674bb580e084685bc6c649f2c38
# Mac=0x0f98156b1ebabd5035c6763db79a10d9bc3096fe
# Chacha20Decrypter=0x2ad22b866c08425bcb5a6b711212d2ba157a5df4
Ibe=$1
Mac=$2
Chacha20Decrypter=$3

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
echo -e "${YELLOW}Deploying the contract...${NC}"
outputDecrypter=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$SECRET_KEY" --wasm-file=./target/wasm32-unknown-unknown/release/decrypter.wasm 2>/dev/null)

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressDecrypter=$(echo "$outputDecrypter" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressDecrypter" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

echo -e "${GREEN}DECRYPTER_CONTRACT_ADDRESS: $addressDecrypter${NC}"

# Initialize the decrypter contract with the helper contracts addresses
echo -e "${YELLOW}Initializing the decrypter contract...${NC}"
initialize_output=$(cast send $addressDecrypter "initialize(string,string,string)" $Ibe $Mac $Chacha20Decrypter --rpc-url $rpc_url --private-key $SECRET_KEY 2>&1)

if [[ $? -ne 0 ]] || [[ $initialize_output == *"revert"* ]]; then
    echo -e "${RED}Initialization failed or reverted.${NC}"
    exit 1
fi

echo -e "${GREEN}Initialization completed successfully.${NC}"

# Run the decrypter example
echo -e "${YELLOW}Running decrypter example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example decrypter "$addressDecrypter" "$SECRET_KEY" 

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Decrypter example ran successfully.${NC}"
else
    echo -e "${RED}Decrypter example failed.${NC}"
    exit 1
fi
