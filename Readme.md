
# Arbitrum Contracts Documentation

This documentation provides an overview of the contracts used for decryption on the Arbitrum platform. To accommodate the contract size limit on Arbitrum, the decryption functionality is distributed across six distinct contracts, each responsible for a specific aspect of the decryption process.

## Overview of Contracts

### decrypter-contract
- **Purpose**: Acts as the central contract coordinating the decryption process and interfacing with other necessary contracts.
- **Functionality**: 
  - Main Function: `decrypt`
  - Parameters: 
    - `uint8[] memory c`: The ciphertext to be decrypted.
    - `uint8[] memory skbytes`: The aggregated decryption key in bytes format.
    - `address ibe_contract`: Address of the IBE (Identity-Based Encryption) contract.
    - `address decrypter_contract`: Address of the decrypter helper contract.
    - `address mac_contract`: Address of the MAC (Message Authentication Code) contract.
  - Returns: `uint8[] memory`: The decrypted data.
  - Additional Notes: The function accepts the addresses of three helper contracts as parameters. These addresses could alternatively be hardcoded.

### ibe-contract
- **Purpose**: Executes IBE decryption to retrieve the file key from the ciphertext.
- **Dependencies**:
  - `ibe-contract-pairing`: Performs the pairing operation.
  - `ibe-contract-hashing`: Conducts the hashing process.
- **Note**: Pairing and hashing are implemented in separate contracts to manage contract size limitations. The address of the two helper contracts are hardcoded in `ibe-contract`.

### chacha20-contract-decrypter
- **Function**: Executes Chacha20Poly1305 decryption using the file key obtained through IBE decryption.

### chacha20-contract-mac
- **Function**: Computes the header MAC (Message Authentication Code) of the ciphertext for verification in the `decrypter-contract`.

## Testing

This section outlines the process for testing the functionality of the Arbitrum Contracts, specifically focusing on an auction contract scenario.

### Overview

In order to test the functionality, a basic implementation of an auction contract has been included in `custom-contract`. 

### Key Functions in `custom-contract`

#### setVars
- **Function**: Initializes the contract by setting various parameters and addresses.
- **Parameters**:
  - `address registry, address decrypter, address ibe_contract, address decrypter_contract, address mac_contract`: Addresses of the helper contracts and necessary components.
  - `uint128 deadline, uint128 id, uint128 fee`: Auction-specific parameters.
- **Usage**: Called at the beginning to set up the auction environment.

#### submitEncBid
- **Function**: Allows submission of an encrypted bid.
- **Parameters**:
  - `uint8[] memory tx`: The encrypted bid data.
  - `string calldata condition`: The condition for bid submission.
- **Returns**: `uint8[] memory`: The status of the bid submission.
- **Usage**: Participants use this to submit their bids in encrypted form. The condition is assumed to be the concatenation of the id and the deadline of the auction in this example contract.

#### submitKey
- **Function**: Submits the aggregated key for decrypting the bids.
- **Parameters**: 
  - `string calldata condition`: The condition which the key is calculated based on it. 
  - `uint8[] memory key`: The aggregated decryption key.
- **Returns**: `uint8[] memory`: The decrypted winner bid.
- **Usage**: Used to decrypt the bids and determine the auction winner.

### Testing Procedure

1. **Script for Simplification**: 
   - Location: `test-script` folder.
   - Function: Automates the deployment of contracts, encryption of bids, and execution of test case. This script gets the private key of a wallet as input. The wallet needs to have some stylus testnet tokens to be able to perform the contract developments and calls. To get the token, the testnet faucet at [Arbitrum Stylus Testnet Faucet](https://bwarelabs.com/faucets/arbitrum-stylus-testnet) can be used.

2. **Pre-requisites**:
   - Repositories: `encrypter` and `ShareGenerator`.
   - Requirement: These repositories need to be built and located alongside the `ArbitrumContracts` directory for the test to function properly.

3. **Execution**:
   - The script initializes the `custom-contract`, submits an encrypted bid, submits the decryption key, and then retrieves the winning bid. These steps are included in the example code of the `test-script/custom-test`.
   - Note: In the current test setup, two bids are submitted. The bid values can be modified through changing the values on lines `81` and `83` of the `setup.sh` script. Moreover, the condition for the encryption and decryption is hardcoded as `1456` assuming the `id = 1` and `deadline = 456`. In order to modify this, the value should be changed on lines `75, 81, and 83` of the script. Also, on lines `106-107, 114, 120, and 126` of the example. (`test-script/custom-test/examples/counter.rs`) 

