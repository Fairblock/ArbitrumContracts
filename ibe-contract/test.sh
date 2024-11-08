#!/bin/bash


# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the absolute path of the directory where the script is located
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to the script directory (where `Cargo.toml` is located)
cd "$script_dir"

# Source the .env file using the absolute path
source "$script_dir/../.env"

# # SECRET_KEY
# SECRET_KEY=<PRIVATE_KEY>
# RPC url
rpc_url=https://sepolia-rollup.arbitrum.io/rpc
# Ibe hashing contract address
# IBE_HASHING=0xdc9dc442a98878d3f4f3cc26b13fb855695565c2

# Accept the address of IBE_HASHING for initialization
IBE_HASHING=$1

# Build the contract
echo -e "${YELLOW}Building the contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release 

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully.${NC}"
else
    echo -e "${RED}Build failed.${NC}"
    exit 1
fi

# Deploy the contract
echo -e "${YELLOW}Deploying the IBE contract...${NC}"
outputIbe=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$SECRET_KEY" --wasm-file=./target/wasm32-unknown-unknown/release/ibe.wasm)

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressIbe=$(echo "$outputIbe" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressIbe" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

echo -e "${GREEN}IBE_CONTRACT_ADDRESS: $addressIbe${NC}"

# Initialize the ibe contract with the ibe IBE_HASHING contract address
echo -e "${YELLOW}Initializing the IBE contract...${NC}"
initialize_output=$(cast send $addressIbe "initialize(string)" $IBE_HASHING --rpc-url $rpc_url --private-key $SECRET_KEY 2>&1)

# if [[ $? -ne 0 ]] || [[ $initialize_output == *"revert"* ]]; then
#     echo -e "${RED}Initialization failed or reverted.${NC}"
#     exit 1
# fi

if [[ $? -ne 0 ]] || [[ $initialize_output == *"revert"* ]]; then
    echo -e "${RED}Initialization failed or reverted. Output:${NC}"
    echo "$initialize_output"
    exit 1
fi


echo -e "${GREEN}Initialization completed successfully.${NC}"

# Run the IBE example
echo -e "${YELLOW}Running IBE example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example ibe "$addressIbe" "$SECRET_KEY"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE example ran successfully.${NC}"
else
    echo -e "${RED}IBE example failed.${NC}"
    exit 1
fi


ok, so with this deploy script

#![cfg_attr(not(feature = "export-abi"), no_main)]
#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::FromStr;
use sha2::Digest;
use stylus_sdk::function_selector;
use stylus_sdk::prelude::sol_interface;
use stylus_sdk::storage::{StorageBool, StorageAddress};
use stylus_sdk::{alloy_primitives::Address, alloy_sol_types, call::Call};
use stylus_sdk::{
    prelude::sol_storage,
    stylus_proc::{entrypoint, external},
};

const BLOCK_SIZE: usize = 32;

sol_storage! {
    #[entrypoint]
    pub struct IBE {
     StorageAddress hasher_addr;
     StorageBool initialized;
    }
}

sol_interface! {

    interface IHasher {
        function verify(uint8[] memory sigma, uint8[] memory msg, uint8[] memory cu) external view returns (bool);
    }
}

/// Performs the IBE decryption
/// The initialize() function can be called once to set the address of the hasher contract
/// 
/// decrypt() function: 
/// # Parameters 
///
/// - r_gid: A Vec<u8> containing the pairing of cu and decryption key.
/// - cv: A Vec<u8> containing the cv part from ciphertext.
/// - cw: A Vec<u8> containing the cw part from ciphertext.
/// - cu: A Vec<u8> containing the cu part from ciphertext.
///
/// # Returns
///
/// - Ok(Vec<u8>): If successful, returns a Vec<u8> containing the plaintext.
/// - Err(stylus_sdk::call::Error): If an error occurs during decryption, it returns an error from the stylus_sdk::call::Error type.
#[external]
impl IBE {
    pub fn initialize(&mut self, hasher_addr: String) -> Result<(), stylus_sdk::call::Error> {
        let initialized = self.initialized.get();
        if initialized {
            return Err(stylus_sdk::call::Error::Revert(
                "Already initialized".as_bytes().to_vec(),
            ));
        }
        self.hasher_addr.set(Address::from_str(&hasher_addr)
        .map_err(|_| {
            stylus_sdk::call::Error::Revert("Invalid hasher address".as_bytes().to_vec())
        })?);
        self.initialized.set(true);
        return Ok(());
    }
    pub fn decrypt(
        &mut self,
        r_gid: Vec<u8>,
        cv: Vec<u8>,
        cw: Vec<u8>,
        cu: Vec<u8>,
    ) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        if cu.len() != 48 || cv.len() > BLOCK_SIZE || cw.len() > BLOCK_SIZE {
            return Err(stylus_sdk::call::Error::Revert(
                "Invalid input length".as_bytes().to_vec(),
            ));
        }

        let sigma = {
            let mut hash = sha2::Sha256::new();

            hash.update(b"IBE-H2");
            hash.update(r_gid);

            let h_r_git: &[u8] = &hash.finalize().to_vec()[0..32];

            xor(h_r_git, &cv)
        };

        let msg = {
            let mut hash = sha2::Sha256::new();
            hash.update(b"IBE-H4");
            hash.update(&sigma);
            let h_sigma = &hash.finalize()[0..BLOCK_SIZE];
            xor(h_sigma, &cw)
        };

        let hasher = IHasher {
            address: *self.hasher_addr,
        };
        let verify_res = hasher
            .verify(Call::new(), sigma.clone(), msg.clone(), cu)
            .map_err(|_| stylus_sdk::call::Error::Revert("Hasher error".as_bytes().to_vec()))?;

        if !verify_res {
            return Err(stylus_sdk::call::Error::Revert(
                "Verfication failed".as_bytes().to_vec(),
            ));
        }

        Ok(msg)
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

--

and this test.sh script for ibe-contract-hashing

#!/bin/bash


# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the absolute path of the directory where the script is located
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to the script directory (where Cargo.toml is located)
cd "$script_dir"

# Source the .env file using the absolute path
source "$script_dir/../.env"

# Private key
# SECRET_KEY=<PRIVATE_KEY>
# RPC url
# rpc_url=https://sepolia-rollup.arbitrum.io/rpc

# Build the contract
echo -e "${YELLOW}Building the contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release 

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully.${NC}"
else
    echo -e "${RED}Build failed.${NC}"
    exit 1
fi

# Deploy the contract
echo -e "${YELLOW}Deploying the IBE hashing contract...${NC}"
# outputIbehashing=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$SECRET_KEY" --wasm-file=./target/wasm32-unknown-unknown/release/stylus-bls.wasm 2>/dev/null)
outputIbehashing=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$SECRET_KEY" --wasm-file=./target/wasm32-unknown-unknown/release/stylus-bls.wasm)

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE hashing contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressIbehashing=$(echo "$outputIbehashing" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressIbehashing" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

# Print contract address with unique marker
echo "IBE_HASHING_CONTRACT_ADDRESS $addressIbehashing"

# Run the hashing example
echo -e "${YELLOW}Running hashing example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example hashing "$addressIbehashing" "$SECRET_KEY"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Hashing example ran successfully.${NC}"
else
    echo -e "${RED}Hashing example failed.${NC}"
    exit 1
fi

--

and this test.sh script for ibe-contract

#!/bin/bash


# Define colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the absolute path of the directory where the script is located
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to the script directory (where Cargo.toml is located)
cd "$script_dir"

# Source the .env file using the absolute path
source "$script_dir/../.env"

# # SECRET_KEY
# SECRET_KEY=<PRIVATE_KEY>
# RPC url
rpc_url=https://sepolia-rollup.arbitrum.io/rpc
# Ibe hashing contract address
# IBE_HASHING=0xdc9dc442a98878d3f4f3cc26b13fb855695565c2

# Accept the address of IBE_HASHING for initialization
IBE_HASHING=$1

# Build the contract
echo -e "${YELLOW}Building the contract...${NC}"
cargo +nightly-2024-05-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=wasm32-unknown-unknown --release 

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully.${NC}"
else
    echo -e "${RED}Build failed.${NC}"
    exit 1
fi

# Deploy the contract
echo -e "${YELLOW}Deploying the IBE contract...${NC}"
outputIbe=$(cargo +nightly-2024-05-20 stylus deploy -e $rpc_url --private-key="$SECRET_KEY" --wasm-file=./target/wasm32-unknown-unknown/release/ibe.wasm)

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE contract deployed successfully.${NC}"
else
    echo -e "${RED}Contract deployment failed.${NC}"
    exit 1
fi

# Extract contract address
addressIbe=$(echo "$outputIbe" | grep "deployed code at address" | awk '{print $5}' | sed 's/\x1b\[[0-9;]*m//g')
if [ -z "$addressIbe" ]; then
    echo -e "${RED}Failed to extract contract address.${NC}"
    exit 1
fi

echo "Debug: Initializing IBE contract with hasher address: $IBE_HASHING"
echo "Debug: Initializing IBE contract with hasher address: "$IBE_HASHING""

echo -e "${GREEN}IBE_CONTRACT_ADDRESS: $addressIbe${NC}"
# echo -e "${GREEN}Debug: IBE_HASHING address is $IBE_HASHING"

# Initialize the ibe contract with the ibe IBE_HASHING contract address
echo -e "${YELLOW}Initializing the IBE contract...${NC}"
initialize_output=$(cast send $addressIbe "initialize(string)" $IBE_HASHING --rpc-url $rpc_url --private-key $SECRET_KEY 2>&1)

# if [[ $? -ne 0 ]] || [[ $initialize_output == *"revert"* ]]; then
#     echo -e "${RED}Initialization failed or reverted.${NC}"
#     exit 1
# fi

if [[ $? -ne 0 ]] || [[ $initialize_output == *"revert"* ]]; then
    echo -e "${RED}Initialization failed or reverted. Output:${NC}"
    echo "$initialize_output"
    exit 1
fi

# try again
# Initialize the ibe contract with the ibe IBE_HASHING contract address
echo -e "${YELLOW}Initializing the IBE contract...${NC}"
initialize_output=$(cast send $addressIbe "initialize(string)" $IBE_HASHING --rpc-url $rpc_url --private-key $SECRET_KEY 2>&1)

# if [[ $? -ne 0 ]] || [[ $initialize_output == *"revert"* ]]; then
#     echo -e "${RED}Initialization failed or reverted.${NC}"
#     exit 1
# fi

if [[ $? -ne 0 ]] || [[ $initialize_output == *"revert"* ]]; then
    echo -e "${RED}Initialization failed or reverted. Output:${NC}"
    echo "$initialize_output"
    exit 1
fi


echo -e "${GREEN}Initialization completed successfully.${NC}"

# Run the IBE example
echo -e "${YELLOW}Running IBE example with the contract address...${NC}"
RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example ibe "$addressIbe" "$SECRET_KEY"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}IBE example ran successfully.${NC}"
else
    echo -e "${RED}IBE example failed.${NC}"
    exit 1
fi


---

what is the format of the passed var for the IBE_HASHING var, is it a string or an address right now? How do I make it work based off of what I just showed you that did work