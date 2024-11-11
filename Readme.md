# Decryption Contracts

Welcome to the Fairblock <> Arbitrum Stylus Integration Tutorial. This repo will be used for the DevCon 2024 Stylus tutorial featuring Fairblock Technologies.

Fairblock is a dynamic confidentiality network that delivers high performance, low overhead, and custom confidential execution to blockchain applications. Dynamic confidentiality unlocks the encrypted economy — onchain applications designed for real-world use cases, like optimizable financial markets, competitive PVP markets like auctions, predictions, and gaming, and privacy-preserving inference.

V1 is live on testnet with bespoke MPEC and threshold identity-based encryption, which offer conditional confidentiality dependent on users’ needs. 

This tutorial focuses on the deployment of decryption contracts, using Arbitrum Stylus, onto an EVM, specifically Sepolia. The decryption contracts allow developers and smart contracts to integrate with Fairblock v1 testnet, thus unlocking the power of a dynamic fondientiality network.

By the end of this tutorial, developers will have:

- Part 1: Deploy their own Fairblock v1 tech stack into an Arbitrum Stylus integrated network, Sepolia. This will result in a deployed `Decrypter` contract on the Sepolia network.
- Part 2: Deploy and test a Sealed Bid Auction smart contract, written in Solidity, with the Decrypter contract from Part 1.
   - The underlying contracts and scripts will provide developers a sense of the integration process with the Stylus contracts and ultimately Fairblock's testnet, Fairyring. 

<!-- Throughout the tutorial, there will be _Context Toggles_ with information that broadens a developer's knowledge of Fairblock technologies. These have been minimized, by default, to ensure that the developer actually jumps into the code as fast as possible. We believe in learning by building :). -->

> If there are any questions, or if you would like to build with the Fairblock ecosystem, please join our discord!

## A Word on Auctions

You may ask, why are we working with a Sealed Bid Auction as an example? Let's touch briefly on the importance of auctions, especially in a landscape that can integrate dynamic confidentiality networks. 

Confidential decentralized auctions safeguard users against exploits like shilling, auctioneer/block proposer last looks, and bid censorship. This approach ensures credible, optimized outcomes for users in terms of execution quality and pricing.

By aligning incentives and enhancing credibility, confidential auctions enable impactful applications across various areas, including intents’ solver auctions, MEV supply chains, single-round Dutch auctions, NFTs, real-world assets (such as real estate, ads, tradfi, or power dispatching systems), restricted access control to boost market efficiency, and innovative SocialFi applications like highest unique bid auctions.

For more information on auctions, refer to:

 • [AFT 2024 Paper](https://drops.dagstuhl.de/storage/00lipics/lipics-vol316-aft2024/LIPIcs.AFT.2024.19/LIPIcs.AFT.2024.19.pdf)
 • [Paradigm Leaderless Auctions](https://www.paradigm.xyz/2024/02/leaderless-auctions)
 • [Arxiv Paper](https://arxiv.org/abs/2404.00475)

Simplicity is the pinnacle of art. Fairblock’s tailored confidentiality schemes deliver robust economic and cryptographic security without adding delays or bandwidth overhead, avoiding the pitfalls of overly complex, general-purpose cryptographic methods.

With all that, let's jump into the tutorial!

## Installation Requirements

To start the project, clone the repo to your local machine using the following CLI command.

Clone the repo onto your local machine and install the submodules: `git clone --recursive https://github.com/Fairblock/ArbitrumContracts.git`

   > NOTE: If you have not installed the submodules, probably because you ran `git clone <repo link>` instead of the CLI command outlined above, you may run into errors when running `forge build` since it is looking for the dependencies for the project. `git submodule update --init --recursive` can be used if you clone the repo without installing the submodules.

Next, make sure you have docker running. If you are new to docker, simply follow the instructions to install Docker Desktop provided on [Docker's website](https://www.docker.com/products/docker-desktop/).

<!-- If operating with a MacOS, at times the default bash version is used, which is around v3.2. The scripts provided in this repo provide versions 4.0 and up. Therefore, you may run into issues when running the commands within this repo. If you do run into some issues, and cannot get your version to be higher than the defualt setting for MacOS, then run homebrew to install the latest version.

One solution may be running a prepend command within a bash script specifying where the newer version of bash is. An example of this is:

```
/opt/homebrew/bin/bash ./deploy_decryption_contracts.sh  
``` -->

### Submodules

There are two submodules used within this repo:

   1. `encrypter` located within the `test-simple-auction-solidity` directory, and used to encrypt the bid values in accordance to the typical UX flow when interacting with Fairyring v1,
   2. `ShareGenerator` located within root of this repo, and used to generate the Master Public Key and Secret Key for encryption, and decryption, respectively.

Please note that The `cyphertext` (encoded tx) is typically done off-chain and submitted on-chain. For the purposes of this tutorial, they are taken care of using the `encrypter` submodule.

> For each of the submodules, it is very important to `cd` into each of them, and run `go build` to construct their binary files that will be used within this repo. 

### 1. Install Rust Nightly Toolchain

Now that the repo is set up and submodules are added, and installed, we will move onto installing Rust Nightly Toolchain. The test scripts use a specific nightly version of Rust. Install and configure Rust by running, at the root:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly-2024-05-20
rustup override set nightly-2024-05-20
```

You also need to install the following target:
```bash
rustup target add wasm32-unknown-unknown
```

### 2. Install Foundry and Cast
Foundry is used for deploying Solidity contracts and interacting with the blockchain. Install Foundry and initialize it at the root:

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```
Verify that both forge and cast are installed correctly:
```bash
forge --version
cast --version
```
### 3. Install Stylus
Stylus is required for deploying Rust contracts. It is this, among other traits, that makes Stylus so powerful. Install it via Cargo at the root:
```bash
cargo install --force cargo-stylus
```
### 4. `.env` Setup
You will need to populate your `.env` with the following (with details on where to get them):

1. `PRIVATE_KEY_1` is a private key associated to a Sepolia Network wallet. Get your's from your own developer wallet. It is used for both deployment of decryption contracts and within integration tests.
2. `PRIVATE_KEY_2` is a private key associated to another Sepolia Network wallet. Get your's from your own developer wallet. It is used for integration tests, specifically with the sealed bid auction example
3. `rpc_url` is simply the rpc_url for the sepolia rollup network. It is provided already within the `env.example`

> The `PUBLIC_KEY` and `SECRET_KEY` used for encryption, and decryption, respectively with the Fairblock v1 tech stack are generated using the `ShareGenerator` submodule as you will see within the tutorial. 

Also note that the `DEPLOYED_DECREYPTER_ADDRESS` is a variable that you will populate later on in the tutorial.

## Deploy the Decryption Contracts

Now that all of the setup has been completed, we will now move onto deploying the Decryption contracts using Stylus on Sepolia. While at the root of the repo, run 
the following commands, note that you must be using a bash version higher than 4.0.

Feel free to learn more about the components making up the full decryption process deployed onto an Arbitrum network. 

<details>
  <summary>The Decryption Contracts Details and Context</summary>

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

</details> 

```bash
./deploy_test_encryption_contracts_verbose.sh
```

What you will see within your terminal are detailed logs revolving around the deployment of contract addresses for the decryption contracts deployed on Sepolia. 

> NOTE: This script is the more verbose bash script. If you would like a less verbose script, please check out `deploy_decryption_contracts.sh`. Although, currently that script is under development. The more verbose script will be presented by Fairblock at the DevCon 2024 Conference. Whereas the other is still undergoing final development.

> Once you have your `DECRYPTER` address, copy and paste the address into the `.env` populating the `DECRYPTION_ADDRESS` var. This is a crucial step required for the integration tests later on in this tutorial.

🎉🎉 Congratulations, you have now launched the encryption contracts necessary to use Fairblock Fairyring v1 technologies on an Arbitrum Stylus integrated test network!

Next, you will test integration with these newly deployed encryption contracts via rust and solidity examples. This highlights the power of using stylus within the Arbitrum network and various smart contract languages, all interfacing simply with a now deployed `Decrypter` contract.

## Run Integration Tests Showcasing the Fairyring v1 Tech Stack on a Arbitrum Stylus Integrated Test Network

There are three different small test examples within this repo:

1. `test-contract-rust`
2. `test-contract-solidity`
3. `test-simple-auction-solidity`

The first two showcase use of rust, and solidity, respectively, for encrypting and decrypting a simple message using the `DECRYPTER` contract that was deployed in the earlier parts of the tutorial.

The third example is a simple variation of a sealed bid auction example deployed using solidity.

To test each one, simply run the `test.sh` scripts within the respective directories.

> You have to `cd` into the respective test example directory that you wish to test before running `./test.sh`

For the sake of this tutorial, we will focus on the third example, test-simple-auction-solidity.

### Sealed Bid Auction Integration Tests: `test-simple-auction-solidity` Directory

The Sealed Bid Auction files can be found within the directory `test-simple-auction-solidity`. Within it, you will see a solidity file, `SealedBidAuctionExample.sol`, and a `test.sh` file. The `test.sh` file essentially runs integration tests against the newly deployed `Decrypter` contract within the Arbitrum Stylus integrated network, Sepolia.

As mentioned before, you must `cd` to within the `test-simple-auction-solidity` directory first.

Now simply run `./test.sh`. You will see that the winning bid is 150, and the respective bidder address.

The world unlocked with the dynamic confidentiality network provided by Fairblock is vast. As the ecosystem onboards more partners, we will write more tutorials and additional content building off of simple examples such as this.

Congratulations! You have now completed the full suite of Arbitrum Stylus <> Fairblock Fairyring v1 quickstart tutorials!