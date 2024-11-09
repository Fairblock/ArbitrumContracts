#!/bin/bash

# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

source ".env"

# Function to call deployment scripts and extract the contract address using a unique marker
deploy_helper_contract() {
    local deploy_script=$1
    local unique_marker=$2
    shift 2  # Shift the parameters to handle additional addresses as arguments


    local output
    if ! output=$(bash "$deploy_script" "$@" 2>&1); then
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
