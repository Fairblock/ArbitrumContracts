#!/bin/bash

# Define color codes for formatting
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
NC="\033[0m"

source "../.env"

echo -e "${BLUE}Starting deployment and interaction script for SealedBidAuctionExample Solidity contract...${NC}"

# Set up configuration variables
echo -e "${YELLOW}Setting up configuration...${NC}"
RPC_URL="https://sepolia-rollup.arbitrum.io/rpc" # actual deployment to Sepolia when done rapid testing
# PRIVATE_KEY=<PRIVATE_KEY>
DECRYPTER=0x6a694d11c59cfc70967eaa47e9f2cd163ac2ae1f # Same decrypter address from Rust script
FEE=10  # Set auction fee, very small so example doesn't brick based on wallet balance

# Get the current block number and set the deadline block to 2 blocks later
CURRENT_BLOCK=$(cast block-number --rpc-url $RPC_URL)
DEADLINE_BLOCK=$((CURRENT_BLOCK + 2))

# Deploy the Solidity contract
echo -e "${YELLOW}Deploying SealedBidAuctionExample contract...${NC}"
OUTPUT=$(forge create --rpc-url $RPC_URL --private-key $PRIVATE_KEY test-simple-auction-solidity/SealedBidAuctionExample.sol:SealedBidAuctionExample --constructor-args $DECRYPTER $DEADLINE_BLOCK $FEE 2>/dev/null)
CONTRACT_ADDRESS=$(echo "$OUTPUT" | grep "Deployed to:" | awk '{print $3}')
echo -e "${GREEN}Contract deployed at address: $CONTRACT_ADDRESS${NC}"

sleep 5

# Submit a bid using mock bid data from the Rust file
echo -e "${YELLOW}Submitting encrypted bid...${NC}"

cd encrypter
bid_value=100
# pk=84dbb7681181e69db71a99e9427344a4478a8b2911f3c6ef36a1891b2e6b4fcfee5c6942ca42502eafe6fc7ec782f60d
Encrypted=$(./encrypter "Random_IBE_ID" $pk $bid_value)
cd ..
BID_DATA=$(python3 convert_to_array.py $Encrypted)

cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "submitEncryptedBid(uint8[])" "$BID_DATA" --value $FEE
echo -e "${GREEN}Encrypted bid submitted!${NC}"

echo -e "${YELLOW}Current block number: ${CURRENT_BLOCK}${NC}"

sleep 32 #wait at least 2 blocks

NEW_BLOCK=$(cast block-number --rpc-url $RPC_URL)
echo -e "${YELLOW}New block number: ${NEW_BLOCK}${NC}"

# Reveal the bid using the secret key from the Rust file
echo -e "${YELLOW}Revealing bid with secret key...${NC}"
SECRET_KEY="[180,94,231,64,60,139,63,77,251,219,173,163,74,124,6,10,129,139,151,186,102,134,86,99,150,127,59,169,18,212,67,132,48,180,58,172,181,219,30,166,33,104,186,198,23,29,20,141,15,107,179,56,147,33,220,105,191,20,32,206,3,203,206,179,228,207,247,100,37,47,155,29,212,118,240,159,79,249,88,182,208,106,20,154,236,61,92,86,122,253,31,5,161,65,125,200]"
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "revealBids(uint8[])" "$SECRET_KEY"
echo -e "${GREEN}Bid revealed!${NC}"

sleep 5

# Check the highest bid and the winner
echo -e "${YELLOW}Checking auction status...${NC}"
HIGHEST_BID=$(cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "highestBid()(uint256)")
WINNER=$(cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "highestBidder()(address)")
echo -e "${GREEN}highestBid: ${HIGHEST_BID} and highetBidder: ${WINNER}${NC}"

# Issue refunds if there are non-winning bids
echo -e "${YELLOW}Issuing refunds to non-winning bidders...${NC}"
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "issueRefunds()"
echo -e "${GREEN}Refunds issued.${NC}"
