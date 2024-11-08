#!/bin/bash

# The purpose of this script is to deploy all necessary contracts to enable encryption with Fairblock technologies on a network integrating Arbitrum Stylus. Encryption contracts are written using rust. This script, alongside this entire repo, is purely for educational purposes. Developers looking to integrate Fairblock technologies and/or develop their own relevant apps must use their own due diligence. For partnerships with the Fairblock ecosystem, please email: TODO: Add email address / contact info.

# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

source ".env"

# SECRET_KEY obtained from Fairyring
# SECRET_KEY=<PRIVATE_KEY>
# RPC url
# rpc_url=https://sepolia-rollup.arbitrum.io/rpc
# Helper contracts addresses
# Ibe=0x24f7d1544e572674bb580e084685bc6c649f2c38
# Mac=0x0f98156b1ebabd5035c6763db79a10d9bc3096fe
# Chacha20Decrypter=0x2ad22b866c08425bcb5a6b711212d2ba157a5df4

# Function to call deployment scripts and extract the contract address using a unique marker
deploy_helper_contract() {
    local deploy_script=$1
    local unique_marker=$2
    shift 2  # Shift the parameters to handle additional addresses as arguments
    local output=$(bash "$deploy_script" "$@")  # Pass all additional arguments to the script

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Deployment script $deploy_script ran successfully.${NC}"
    else
        echo -e "${RED}Deployment of $deploy_script failed.${NC}"
        exit 1
    fi

    # Extract the contract address using the unique marker from the output
    local contract_address=$(echo "$output" | grep "$unique_marker" | awk '{print $NF}' | sed 's/\x1b\[[0-9;]*m//g')
    if [ -z "$contract_address" ]; then
        echo -e "${RED}Failed to extract contract address for marker $unique_marker from $deploy_script output.${NC}"
        exit 1
    fi

    echo "$contract_address"
}

# Call the helper deployment scripts

# # Deployment scripts and their unique markers
# declare -a deploy_scripts=(
#     "path/to/ibe_hashing/test.sh IBE_HASHING_CONTRACT_ADDRESS"
#     "path/to/ibe/test.sh IBE_CONTRACT_ADDRESS"
#     "path/to/chacha20_mac/test.sh CHACHA20_MAC_CONTRACT_ADDRESS"
#     "path/to/chacha20_decrypter/test.sh CHACHA20_DECRYPTER_CONTRACT_ADDRESS"
#     "path/to/decrypter/test.sh DECRYPTER_CONTRACT_ADDRESS"
# )

# Variables to store the addresses of deployed contracts
declare -A contract_addresses

# Deploy IBE_HASHING first
contract_addresses["IBE_HASHING"]=$(deploy_helper_contract "./ibe-contract-hashing/test.sh" "IBE_HASHING_CONTRACT_ADDRESS")
echo -e "${GREEN}IBE_HASHING address: ${contract_addresses["IBE_HASHING"]}${NC}"

# Deploy IBE using the IBE_HASHING address for initialization
contract_addresses["IBE"]=$(deploy_helper_contract "./ibe-contract/test.sh" "IBE_CONTRACT_ADDRESS" "${contract_addresses["IBE_HASHING"]}")
echo -e "${GREEN}IBE address: ${contract_addresses["IBE"]}${NC}"

# Deploy CHACHA20_MAC without any dependencies
contract_addresses["CHACHA20_MAC"]=$(deploy_helper_contract "./chacha20-contract-mac/test.sh" "CHACHA20_MAC_CONTRACT_ADDRESS")
echo -e "${GREEN}CHACHA20_MAC address: ${contract_addresses["CHACHA20_MAC"]}${NC}"

# Deploy CHACHA20_DECRYPTER without any dependencies
contract_addresses["CHACHA20_DECRYPTER"]=$(deploy_helper_contract "./chacha20-contract-decrypter/test.sh" "CHACHA20_DECRYPTER_CONTRACT_ADDRESS")
echo -e "${GREEN}CHACHA20_DECRYPTER address: ${contract_addresses["CHACHA20_DECRYPTER"]}${NC}"

# Deploy DECRYPTER using IBE, CHACHA20_MAC, and CHACHA20_DECRYPTER for initialization
contract_addresses["DECRYPTER"]=$(deploy_helper_contract "./decrypter-contract/test.sh" "DECRYPTER_CONTRACT_ADDRESS" \
    "${contract_addresses["IBE"]}" "${contract_addresses["CHACHA20_MAC"]}" "${contract_addresses["CHACHA20_DECRYPTER"]}")
echo -e "${GREEN}DECRYPTER address: ${contract_addresses["DECRYPTER"]}${NC}"
