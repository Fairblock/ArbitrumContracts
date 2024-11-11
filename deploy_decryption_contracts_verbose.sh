#!/bin/bash

# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

source ".env"

# Deploy IBE_HASHING 
cd ibe-contract-hashing 
echo -e "${YELLOW}Building the IBE_HASHING contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release 
echo -e "${YELLOW}Deploying the IBE hashing contract...${NC}"
outputIbehashing=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$PRIVATE_KEY_1" --wasm-file=./target/wasm32-unknown-unknown/release/ibe-contract-hashing.wasm)
addressIbehashing=$(echo "$outputIbehashing" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
echo -e "${GREEN}IBE_HASHING_CONTRACT_ADDRESS $addressIbehashing"
cd ..

# Deploy IBE_CONTRACT
cd ibe-contract 
echo -e "${YELLOW}Building the IBE contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release 
echo -e "${YELLOW}Deploying the IBE contract...${NC}"
outputIbe=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$PRIVATE_KEY_1" --wasm-file=./target/wasm32-unknown-unknown/release/ibe.wasm)
addressIbe=$(echo "$outputIbe" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
echo -e "${GREEN}IBE_CONTRACT_ADDRESS $addressIbe"
echo -e "${YELLOW}Initializing the IBE contract...${NC}"
initialize_ibe_output=$(cast send $addressIbe "initialize(string)" $addressIbehashing --rpc-url $rpc_url --private-key $PRIVATE_KEY_1)
echo "${initialize_ibe_output}"
echo -e "${YELLOW}Initialized the IBE contract" 
cd ..

# Deploy CHACHA20_MAC 
cd chacha20-contract-mac
echo -e "${YELLOW}Building the CHACHA20_MAC contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release
echo -e "${YELLOW}Deploying the CHACHA_20_MAC contract...${NC}"
outputmac=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$PRIVATE_KEY_1" --wasm-file=./target/wasm32-unknown-unknown/release/chacha20mac.wasm)
addressmac=$(echo "$outputmac" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
echo -e "${GREEN}CHACHA20_MAC_ADDRESS $addressmac" 
cd ..

# Deploy CHACHA20_DECRYPTER
cd chacha20-contract-decrypter
echo -e "${YELLOW}Building the CHACHA20_DECRYPTER contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release
echo -e "${YELLOW}Deploying the CHACHA20_DECRYPTER contract...${NC}"
outputchachadecrypter=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$PRIVATE_KEY_1" --wasm-file=./target/wasm32-unknown-unknown/release/chacha20.wasm)
addresschachadecrypter=$(echo "$outputchachadecrypter" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
echo -e "${GREEN}CHACHA20_DECRYPTER_ADDRESS $addresschachadecrypter"
cd ..

# Deploy DECRYPTER 
cd decrypter-contract
echo -e "${YELLOW}Building the DECRYPTER contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release
echo -e "${YELLOW}Deploying the DECRYPTER contract...${NC}"
outputdecrypter=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$PRIVATE_KEY_1" --wasm-file=./target/wasm32-unknown-unknown/release/decrypter.wasm)
addressdecrypter=$(echo "$outputdecrypter" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
echo -e "${GREEN}DECRYPTER_ADDRESS $addressdecrypter"
echo -e "${YELLOW}Initializing the decrypter contract...${NC}"
initialize_decrypter_output=$(cast send $addressdecrypter "initialize(string,string,string)" $addressIbe $addressmac $addresschachadecrypter --rpc-url $rpc_url --private-key $PRIVATE_KEY_1)
echo "${initialize_decrypter_output}"
echo -e "${GREEN}Decrypter initialization completed successfully.${NC}"
cd ..
