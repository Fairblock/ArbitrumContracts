Run the command:

```
#!/bin/bash
set -e  # Stop on any error

# Clone the repo and wait for completion
git clone https://github.com/Fairblock/ArbitrumContracts.git && cd ArbitrumContracts

# Switch to the desired branch
git checkout feat/update-with-second-bid

# Initialize and update submodules
git submodule init && git submodule update --recursive --init

# Verify submodules are installed
git submodule status

# Build submodules
cd encrypter && go build && cd ../ShareGenerator && go build && cd ..

# Install Rust and target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env  # Load Rust environment
rustup install nightly-2024-05-20
rustup override set nightly-2024-05-20
rustup target add wasm32-unknown-unknown

# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
source $HOME/.bashrc  # Adjust based on the user's shell (e.g., .zshrc)
foundryup
forge --version
cast --version

```

Update your `.env`

Now, run the next batch of commands to deploy the decryption contracts, and then also run the integration tests for the simple sealed bid auction example.

You can do this simply by running the `StylusTutorial.sh` script.

Run:

```
./StylusTutorial.sh
```
