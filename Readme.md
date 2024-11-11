# Decryption Contracts

## Installation Requirements

To start the project, clone the repo to your local machine using the following CLI command.

Clone the repo onto your local machine and install the submodules: `git clone --recursive https://github.com/Fairblock/ArbitrumContracts.git`

   > NOTE: If you have not installed the submodules, probably because you ran `git clone <repo link>` instead of the CLI command outlined above, you may run into errors when running `forge build` since it is looking for the dependencies for the project. `git submodule update --init --recursive` can be used if you clone the repo without installing the submodules.

If operating with a MacOS, at times the default bash version is used, which is around v3.2. The scripts provided in this repo provide versions 4.0 and up. Therefore, you may run into issues when running the commands within this repo. If you do run into some issues, and cannot get your version to be higher than the defualt setting for MacOS, then run homebrew to install the latest version.

One solution may be running a prepend command within a bash script specifying where the newer version of bash is. An example of this is:

```
/opt/homebrew/bin/bash ./deploy_decryption_contracts.sh  
```

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
Stylus is required for deploying Rust contracts. Install it via Cargo at the root:
```bash
cargo install --force cargo-stylus
```
### 4. `.env` Setup
You will need to populate your `.env` with the following (with details on where to get them):

1. `PRIVATE_KEY_1` is a private key associated to a Sepolia Network wallet. Get your's from your own developer wallet.
2. `PRIVATE_KEY_2` is a private key associated to another Sepolia Network wallet. Get your's from your own developer wallet.
3. `rpc_url` is simply the rpc_url for the sepolia rollup network.

> The `PUBLIC_KEY` and `SECRET_KEY` used for encryption, and decryption, respectively with the Fairblock v1 tech stack are generated using the `ShareGenerator` submodule as you will see within the tutorial. 

## Deploy the Encryption Contracts

While at the root of the repo, run the following commands, note that you must be using a bash version higher than 4.0.

```bash
./deploy_test_encryption_contracts.sh
```

What you will see within your terminal are contract addresses for the encryption contracts deployed onto the Arbitrum Stylus integrated network, Sepolia. The output will look something like this (these addresses are purely educational and deployed on Sepolia test network):

```bash
IBE_HASHING address: 0xf1b77277366e3b37e53cd04de4562c1b06eabfc1
IBE address: 0xfff37f682789b4b7e210090fa60b95a33d1c4a24
CHACHA20_MAC address: 0x1947b8d6b5178110dffc202440b35b39209dd748
CHACHA20_DECRYPTER address: 0x1c09eae982d7d4c37add657b310775297c1ebedd
DECRYPTER address: 0xfce7f2686365aa7528bfc4a078c88a1ab5da7ca7
```

> NOTE: This script is the less verbose bash script. If you would like to have more details consoled to your terminal, please check out `deploy_decryption_contracts_verbose.sh`. This script will be also presented by Fairblock at the DevCon 2024 Conference.

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

Let's walk through each one.

### TODO: `test-contract-rust`

### TODO: `test-contract-solidity`

### TODO: `test-simple-auction-solidity`

This tutorial showcases a couple of different things in addition to a sealed bid auction example.

Now simply run `./test.sh` as you did in the other simple integration test directories. You will see that the winning bid is 150, and the respective bidder address.

Congratulations! You have now completed the full suite of Arbitrum Stylus <> Fairblock Fairyring v1 quickstart tutorials!