#!/bin/bash

# Define color codes for formatting
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
NC="\033[0m"

# Get the absolute path of the directory where the script is located
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to the script directory (where `Cargo.toml` is located)
cd "$script_dir"

# Source the .env file using the absolute path
source "$script_dir/../.env"

echo -e "${BLUE}Starting deployment and interaction script for SealedBidAuctionExample Solidity contract...${NC}"

# Set up configuration variables
echo -e "${YELLOW}Setting up configuration...${NC}"
FEE=10  # Set auction fee, very small so example doesn't brick based on wallet balance

# Get the current block number and set the deadline block to 2 blocks later
CURRENT_BLOCK=$(cast block-number --rpc-url $rpc_url)
DEADLINE_BLOCK=$((CURRENT_BLOCK + 10))

# Deploy the Solidity contract
echo -e "${YELLOW}Deploying SealedBidAuctionExample contract...${NC}"
OUTPUT=$(forge create --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 test-simple-auction-solidity/SealedBidAuctionExample.sol:SealedBidAuctionExample --constructor-args $DEPLOYED_DECRYPTER_ADDRESS $DEADLINE_BLOCK $FEE 2>/dev/null)
CONTRACT_ADDRESS=$(echo "$OUTPUT" | grep "Deployed to:" | awk '{print $3}')
echo -e "${GREEN}Contract deployed at address: $CONTRACT_ADDRESS${NC}"

sleep 5

# Master public key needs to be generated to encrypt the bid data.
PUBLIC_KEY_NEW=$(../sharegenerator) # TODO: not sure if there are any inputs needed
echo -e "${YELLOW}NEW PUBLIC KEY GENERATED: $PUBLIC_KEY_NEW"


# User 1 submits a bid using mock bid data from the Rust file
echo -e "${YELLOW}Submitting encrypted bid from user #1...${NC}"

cd encrypter
bid_value=100
Encrypted=$(./encrypter "Random_IBE_ID" $PUBLIC_KEY_NEW $bid_value)
cd ..
BID_DATA=$(python3 convert_to_array.py $Encrypted)

cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "submitEncryptedBid(uint8[])" "$BID_DATA" --value $FEE
echo -e "${GREEN}Encrypted bid submitted!${NC}"

echo -e "${YELLOW}Current block number: ${CURRENT_BLOCK}${NC}"

# User 2 submits a bid using mock bid data from the Rust file
echo -e "${YELLOW}Submitting encrypted bid from user #2...${NC}"

cd encrypter
bid_value=150
Encrypted=$(./encrypter "Random_IBE_ID" $PUBLIC_KEY_NEW $bid_value)
cd ..
BID_DATA=$(python3 convert_to_array.py $Encrypted)

cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_2 $CONTRACT_ADDRESS "submitEncryptedBid(uint8[])" "$BID_DATA" --value $FEE
echo -e "${GREEN}Encrypted bid submitted!${NC}"

echo -e "${YELLOW}Current block number: ${CURRENT_BLOCK}${NC}"

sleep 32 #wait at least 2 blocks

NEW_BLOCK=$(cast block-number --rpc-url $rpc_url)
echo -e "${YELLOW}New block number: ${NEW_BLOCK}${NC}"

# Testing use of the keyshare from ShareGenerator and formatting it in a way that is needed to test with
echo -e "${YELLOW}Keyshare obtained from ShareGenerator: $KEYSHARE"
SECRET_KEY_NEW=$(python3 convert_to_array.py $KEYSHARE)
echo -e "${YELLOW}Formatted Keyshare obtained from convert_to_array: $SECRET_KEY_NEW"

# TODO: THIS NOW USES THE SECRET_KEY_NEW AND PUBLIC KEY GENERATED FROM SHARE GENERATOR REPO. IT IS NOT WORKING THOUGH.
# newly generated via share generator repo
echo -e "${YELLOW}Revealing bid with secret key...${NC}"
cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "revealBids(uint8[])" "$SECRET_KEY_NEW"
echo -e "${GREEN}Bid revealed!${NC}"

sleep 5

# Check the highest bid and the winner
echo -e "${YELLOW}Checking auction status...${NC}"
HIGHEST_BID=$(cast call --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "highestBid()(uint256)")
WINNER=$(cast call --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "highestBidder()(address)")
echo -e "${GREEN}highestBid: ${HIGHEST_BID} and highestBidder: ${WINNER}${NC}"

# Issue refunds if there are non-winning bids
echo -e "${YELLOW}Issuing refunds to non-winning bidders...${NC}"
cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "issueRefunds()"
echo -e "${GREEN}Refunds issued.${NC}"
