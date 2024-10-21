# Decryption Contracts
This repository includes the contracts implemented to perform the IBE decryption process on Arbitrum chain. 

## Contract Description and Gas Consumption

The decryption process involves 5 contracts. Below is a breakdown of each contract and their respective gas consumption:

### 1. **IBE Contract (Hashing)**
- **Functionality:** Verifies the correctness of the ciphertext based on the Boneh-Franklin Identity-Based Encryption (BF-IBE) algorithm. It calculates a hash over the message and sigma, multiplies it by `P`, and verifies that the result matches the `U` component in the ciphertext.
- **Gas Consumption:** ~1,587,000
  - **Key Contributor:** Scalar and G1 point multiplication, consuming 1,366,619 gas.

### 2. **IBE Contract**
- **Functionality:** Decrypts the ciphertext and recovers the message (which is the symmetric key for the second layer of encryption). It leverages the IBE Contract (Hashing) for ciphertext validation.
- **Gas Consumption:** ~1,742,000(~1,587,000 of this comes from the IBE Contract (Hashing))
  - **Note:** The majority of the gas consumption comes from the hashing contract.

### 3. **ChaCha20 MAC Contract**
- **Functionality:** Computes the MAC for the ciphertext header using the key and ciphertext body.
- **Gas Consumption:** ~72,000
  - **Note:** Minimal gas usage.

### 4. **ChaCha20 Decryption Contract**
- **Functionality:** Performs symmetric key decryption using the provided key and returns the plaintext.
- **Gas Consumption:** ~55,000
  - **Note:** Minimal gas usage.

### 5. **Decryption Interface Contract**
- **Functionality:** Serves as the main interface for the decryption process. It accepts the decryption key and ciphertext, invoking the appropriate contracts to perform the full decryption.
- **Gas Consumption:** ~9,189,000
  - **Breakdown:**
    - IBE, MAC, and ChaCha20 contracts: As described above.
    - ~1,565,000: Deserializing the decryption key.
    - ~5,445,000: Pairing operation.




## Testing
In order to test each contract separately, there is a simple example implemented inside the `examples` folder in each contract's directory. There is also a `test.sh` script for each contract that deploys the contract and runs the example with it. 

Moreover, to demonstrate how the contracts can be used from inside another contract (e.g. an application contract), two test contracts `test-contract-rust` and `test-contract-solidity` are provided. The rust version implements a basic auction application where bids are encrypted and will be decrypted using the decrypter contracts once the decryption key is provided. For simplicity of the testing process, the unnecessary checks are eliminated in the contract. 
The solidity version implements a simple example of a contract which allows for submission of encrypted messages and decryption of them once the key is submitted.
Both examples can be tested using the provided `test.sh` files in their corresponding folders.

For all tests, the contracts will be deployed on Arbitrum testnet by default but the RPC url can be modified in the `test.sh` and example files. Moreover, the private key (`sk`) should be set to the private key of an account on Arbitrum testnet which has enough funds to cover the gas fees for deploying and calling the contracts.
Note that the addresses for the deployed contracts are hardcoded in the `decrypter-contract`, `ibe-contract`, and also in the test scripts for the two test contracts. They can be replaced with newly deployed contracts if needed.
