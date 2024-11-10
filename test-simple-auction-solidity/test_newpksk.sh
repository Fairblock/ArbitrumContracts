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

# User 1 submits a bid using mock bid data from the Rust file
echo -e "${YELLOW}Submitting encrypted bid from user #1...${NC}"

cd encrypter
bid_value=100
Encrypted=$(./encrypter "Random_IBE_ID" $PUBLIC_KEY_2 $bid_value)
cd ..
BID_DATA=$(python3 convert_to_array.py $Encrypted)

cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "submitEncryptedBid(uint8[])" "$BID_DATA" --value $FEE
echo -e "${GREEN}Encrypted bid submitted!${NC}"

echo -e "${YELLOW}Current block number: ${CURRENT_BLOCK}${NC}"

# User 2 submits a bid using mock bid data from the Rust file
echo -e "${YELLOW}Submitting encrypted bid from user #2...${NC}"

cd encrypter
bid_value=150
Encrypted=$(./encrypter "Random_IBE_ID" $PUBLIC_KEY_2 $bid_value)
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

# TODO: THIS IS JUST RAW SECRET KEY ATTEMPTED TO BE NEWLY GENERATED... HASH IS TROUBLESHOOTING THIS. I GOT THIS FROM USING THE SHARE GENERATOR REPO. I THEN USED THE CORRESPONDING PUBLIC KEY IN THE .ENV, BUT SADLY IT IS NOT WORKING RIGHT NOW WITH THESE NEWLY GENERATED KEYS.
# newly generated via share generator repo
echo -e "${YELLOW}Revealing bid with secret key...${NC}"
SECRET_KEY_2="[135, 137, 141, 215, 238, 12, 175, 195, 47, 158, 244, 88, 147, 223, 107, 229, 251, 165, 200, 42, 230, 51, 50, 61, 106, 124, 87, 78, 71, 71, 104, 152, 188, 163, 92, 228, 213, 83, 182, 182, 179, 27, 248, 232, 243, 40, 249, 71, 15, 206, 136, 194, 50, 255, 229, 27, 206, 63, 144, 184, 39, 213, 10, 187, 252, 177, 220, 46, 255, 94, 58, 233, 238, 119, 137, 237, 254, 132, 89, 63, 245, 110, 36, 171, 30, 170, 88, 97, 238, 197, 25, 245, 235, 74, 74, 138]"
cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "revealBids(uint8[])" "$SECRET_KEY_2"
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
