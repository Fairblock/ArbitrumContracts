#!/bin/bash

# Define color codes for formatting
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
NC="\033[0m"

echo -e "${BLUE}Starting deployment and interaction script for SealedBidAuctionExample Solidity contract...${NC}"

# Set up configuration variables
echo -e "${YELLOW}Setting up configuration...${NC}"
RPC_URL="https://sepolia-rollup.arbitrum.io/rpc" # actual deployment to Sepolia when done rapid testing
PRIVATE_KEY=0xf23f1dda5f386eab6af2240e8bf33d92044c1af6e59124e0dbe85aa92b2c3300
DECRYPTER=0xcb5aadb5bf01d6b685219e98d7c5713b7ac73042 # Same decrypter address from Rust script
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
BID_DATA="[97,103,101,45,101,110,99,114,121,112,116,105,111,110,46,111,114,103,47,118,49,10,45,62,32,100,105,115,116,73,66,69,10,106,119,117,102,81,101,115,53,75,71,69,75,104,67,104,109,88,79,101,49,102,65,43,56,107,57,109,52,54,71,113,83,76,111,108,98,48,74,67,113,83,75,116,82,120,72,113,105,50,107,51,70,108,76,114,101,107,114,90,106,81,52,97,117,10,103,86,111,99,113,66,106,90,101,109,105,82,66,54,86,79,83,70,54,110,74,113,117,43,84,104,115,117,81,67,86,117,103,72,76,86,120,48,100,90,98,70,78,56,48,84,52,53,66,108,77,101,43,122,57,85,90,50,97,111,115,110,106,71,10,104,53,111,79,67,57,51,84,90,98,69,53,79,79,97,83,85,112,111,43,69,81,10,45,45,45,32,105,98,67,76,115,81,47,86,101,53,52,116,80,116,106,99,49,85,88,88,98,75,69,53,84,90,104,56,113,100,102,89,105,77,57,107,53,53,70,100,107,108,107,10,216,219,176,112,82,16,95,62,198,100,66,28,145,63,103,141,49,246,71,167,195,230,38,195,96,226,12,13,21,49,85,119,205,78,198]"
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
HIGHEST_BID=$(cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "highestBid()")
WINNER=$(cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "highestBidder()")
echo -e "${GREEN}Highest bid: ${HIGHEST_BID} by: ${WINNER}${NC}"

# Issue refunds if there are non-winning bids
echo -e "${YELLOW}Issuing refunds to non-winning bidders...${NC}"
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "issueRefunds()"
echo -e "${GREEN}Refunds issued.${NC}"
