set -e  # Stop on any error

chmod +x StylusTutorial.sh

# Prompt the user to update their .env file
echo "Please copy over two PRIVATE_KEYs from Sepolia wallets (they can be new and empty) and add it to the .env file under the 'PRIVATE_KEY_1' and 'PRIVATE_KEY_2' variables."
read -p "Press Enter after updating the .env file..."

# Deploy the decryption contracts
echo "Deploying decryption contracts..."
./deploy_decryption_contracts_verbose.sh

# Prompt the user to add the DECRYPTER address to the .env file
echo "Please copy the DECRYPTER address from the output and add it to the .env file under the 'DEPLOYED_DECRYPTER_ADDRESS' variable."
read -p "Press Enter after updating the .env file..."

# Navigate to the test-simple-auction-solidity directory and run the test
echo "Navigating to the 'test-simple-auction-solidity' directory and running the test..."
cd test-simple-auction-solidity
./test.sh

echo "Integration tests completed successfully!"
