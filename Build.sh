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