#!/bin/bash

# Define color codes
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
NC="\033[0m" # No Color

# Define bold text
BOLD="\033[1m"

echo -e "${BLUE}Starting the deployment and interaction script...${NC}"


echo -e "${YELLOW}Setting up configuration...${NC}"
RPC_URL="https://sepolia-rollup.arbitrum.io/rpc"
PRIVATE_KEY=<PRIVATE_KEY>
DECRYPTER=0x175243d50f99d494a9e8349529ca240e7c7e8586


echo -e "${YELLOW}Deploying contract...${NC}"
OUTPUT=$(forge create --rpc-url $RPC_URL --private-key $PRIVATE_KEY test-contract-solidity/Encrypted.sol:MessageStorage --constructor-args $DECRYPTER 2>/dev/null)
Contract=$(echo "$OUTPUT" | grep "Deployed to:" | awk '{print $3}')
echo -e "${GREEN}Deployed contract at address: ${Contract}${NC}"
sleep 3

echo -e "${YELLOW}Submitting encrypted message to contract...${NC}"
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $Contract "submitMessage(uint8[],string)()" "[97, 103, 101, 45, 101, 110, 99, 114, 121, 112, 116, 105, 111, 110, 46, 111, 114, 103, 47,118, 49, 10, 45, 62, 32, 100, 105, 115, 116, 73, 66, 69, 10, 114, 97, 103, 71, 90, 43, 48,83, 48, 75, 54, 122, 120, 55, 68, 121, 54, 70, 115, 49, 47, 111, 86, 109, 81, 75, 57, 88,100, 78, 122, 106, 75, 85, 70, 57, 120, 116, 114, 89, 49, 114, 122, 119, 116, 75, 80, 105,69, 109, 113, 100, 79, 116, 100, 115, 103, 81, 79, 112, 101, 97, 111, 78, 10, 54, 110, 56,69, 110, 55, 72, 51, 79, 56, 120, 97, 109, 77, 117, 103, 103, 52, 106, 102, 74, 78, 79, 53,101, 116, 85, 102, 51, 119, 75, 88, 87, 103, 104, 54, 75, 76, 79, 75, 43, 75, 89, 101, 69,119, 70, 81, 83, 81, 43, 47, 100, 118, 52, 52, 57, 79, 110, 104, 111, 52, 98, 121, 10, 113,106, 87, 100, 116, 117, 114, 112, 43, 115, 47, 100, 81, 74, 100, 109, 88, 99, 43, 56, 104,65, 10, 45, 45, 45, 32, 114, 52, 84, 51, 65, 66, 74, 54, 79, 103, 119, 69, 105, 55, 90, 78,68, 108, 78, 88, 75, 85, 55, 82, 120, 122, 67, 102, 116, 52, 105, 68, 74, 103, 119, 109,108, 72, 103, 78, 75, 100, 99, 10, 37, 61, 47, 1, 244, 206, 42, 96, 20, 9, 7, 125, 207, 71,69, 210, 104, 143, 189, 62, 0, 194, 29, 184, 189, 149, 107, 25, 206, 151, 8, 95, 30, 144,61, 203, 218, 96, 122, 237, 116, 192, 86]" "test"
sleep 10

echo -e "${YELLOW}Submitting key to contract...${NC}"
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $Contract "submitKey(string,uint8[])()" "test" "[180, 94, 231, 64, 60, 139, 63, 77, 251, 219, 173, 163, 74, 124, 6, 10, 129, 139, 151, 186,102, 134, 86, 99, 150, 127, 59, 169, 18, 212, 67, 132, 48, 180, 58, 172, 181, 219, 30, 166,33, 104, 186, 198, 23, 29, 20, 141, 15, 107, 179, 56, 147, 33, 220, 105, 191, 20, 32, 206,3, 203, 206, 179, 228, 207, 247, 100, 37, 47, 155, 29, 212, 118, 240, 159, 79, 249, 88,182, 208, 106, 20, 154, 236, 61, 92, 86, 122, 253, 31, 5, 161, 65, 125, 200]"
sleep 10 

echo -e "${YELLOW}Calling contract to check messages...${NC}"
result=$(cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $Contract "checkMessages(string)(uint8[][])" "test")


ascii_values=$(echo "$result" | grep -oP '\d+')

echo -e "${YELLOW}Decoded message from contract:${NC}"
for i in $ascii_values; do
  printf "\\$(printf '%03o' "$i")"
done

echo