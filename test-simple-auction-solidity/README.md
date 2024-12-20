# Confidential Auction Example

This subdirectory focuses on the simple auction example using Solidity. It highlights:

- General developer flow for a solidity developer, where transaction flow includes:
   - Submission of encrypted data in smart contract storage, 
   - Call `Decrypter` contract (that was deployed using Stylus) to `decrypt` respective messages, 
   - Carry out whatever necessary with decrypted message details.

> Make sure you are in the subdirectory: `test-simple-auction-solidity`, and ensure that you have foundry installed for this repo.

Update `.test.sh` with your `PRIVATE_KEY`.

`PRIVATE_KEY=0x123...`

Run the test script on the Arbitrum Sepolia tesnet.

`./test.sh`
