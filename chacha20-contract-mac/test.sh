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
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release > /dev/null 2>&1

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully.${NC}"
else
    echo -e "${RED}Build failed.${NC}"
    exit 1
fi

# Deploy the contract
echo -e "${YELLOW}Deploying the contract...${NC}"
outputmac=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$PRIVATE_KEY_1" --wasm-file=./target/wasm32-unknown-unknown/release/chacha20mac.wasm 2>/dev/null)

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressmac=$(echo "$outputmac" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressmac" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

echo -e "${GREEN}CHACHA20_MAC_CONTRACT_ADDRESS: $addressmac${NC}"

# Run the MAC example
echo -e "${YELLOW}Running MAC example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example mac "$addressmac" "$PRIVATE_KEY_1"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}MAC example ran successfully.${NC}"
else
    echo -e "${RED}MAC example failed.${NC}"
    exit 1
fi
