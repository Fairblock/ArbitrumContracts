#!/bin/bash

# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# MODIFY THIS
address_decrypter=0x99e63278e95b3a04767b6a6e17d94c4eef892304


# use ShareGenerator to generate the master public and secret keys 
cd ShareGenerator 

output=$(./ShareGenerator generate 1 1 | jq '.')
master_public_key=$(echo "$output" | jq -r '.MasterPublicKey')
share_value=$(echo "$output" | jq -r '.Shares[0].Value')

echo -e "${GREEN}MasterPublicKey: $master_public_key${NC}"
echo -e "${GREEN}MasterSecretKey: $share_value${NC}"

identity="STYLUS_WORKSHOP"
echo -e "${GREEN}Identity: $identity${NC}"

derivedkeyoutput=$(./ShareGenerator derive $share_value 0 $identity | jq '.')
derived_secret_key=$(echo "$derivedkeyoutput" | jq -r '.KeyShare' )
echo $derived_secret_key

cd ..

source ".env"

cd test-simple-auction-solidity

# convert the mpk to uint8[]

mpk_array=$(python3 convert_to_array.py $master_public_key)
echo -e "${YELLOW}MPK array: $mpk_array${NC}"

# convert the derived_secret_key to uint8[]

dsk_array=$(python3 convert_to_array.py $derived_secret_key)
echo -e "${YELLOW}DSK array: $dsk_array${NC}"

# deploy the auction contract

FEE=10  # Set auction fee, very small so example doesn't brick based on wallet balance

CURRENT_BLOCK=$(cast block-number --rpc-url $rpc_url)
DEADLINE_BLOCK=$((CURRENT_BLOCK + 50))

echo "Current block ${CURRENT_BLOCK}"
echo "Deadline block ${DEADLINE_BLOCK}"


# Deploy the Solidity contract
echo -e "${YELLOW}Deploying SealedBidAuctionExample contract...${NC}"
OUTPUT=$(forge create --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 test-simple-auction-solidity/SealedBidAuctionExample.sol:SealedBidAuctionExample --constructor-args $address_decrypter $DEADLINE_BLOCK $FEE 2>/dev/null)
CONTRACT_ADDRESS=$(echo "$OUTPUT" | grep "Deployed to:" | awk '{print $3}')
echo -e "${GREEN}Contract deployed at address: $CONTRACT_ADDRESS${NC}"

# make some bids 

echo -e "${YELLOW}Submitting encrypted bid from user #1...${NC}"

bid_value1=100
encrypted_1=$(./encrypter/encrypter $identity $master_public_key $bid_value1)
bid_array1=$(python3 convert_to_array.py $encrypted_1)

cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "submitEncryptedBid(uint8[])" "$bid_array1" --value $FEE
echo -e "${GREEN}Encrypted bid submitted!${NC}"

echo -e "${YELLOW}Submitting encrypted bid from user #2...${NC}"

bid_value2=150
encrypted_2=$(./encrypter/encrypter $identity $master_public_key $bid_value2)
bid_array2=$(python3 convert_to_array.py $encrypted_2)

cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_2 $CONTRACT_ADDRESS "submitEncryptedBid(uint8[])" "$bid_array2" --value $FEE
echo -e "${GREEN}Encrypted bid submitted!${NC}"

TEMP_BLOCK=$CURRENT_BLOCK

# Loop until TEMP_BLOCK reaches deadline 
while [ "$TEMP_BLOCK" -lt "$DEADLINE_BLOCK" ]; do
    # Get the current block number and update TEMP_BLOCK
    TEMP_BLOCK=$(cast block-number --rpc-url "$rpc_url")
    echo "current block: $TEMP_BLOCK"

    # Sleep for 1 second
    sleep 1
done

echo -e "${YELLOW}Revealing bid with secret key...${NC}"
cast send --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "revealBids(uint8[])" "$dsk_array"

echo -e "${YELLOW}Checking auction status...${NC}"
HIGHEST_BID=$(cast call --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "highestBid()(uint256)")
WINNER=$(cast call --rpc-url $rpc_url --private-key $PRIVATE_KEY_1 $CONTRACT_ADDRESS "highestBidder()(address)")
echo -e "${GREEN}highestBid: ${HIGHEST_BID} and highestBidder: ${WINNER}${NC}"




