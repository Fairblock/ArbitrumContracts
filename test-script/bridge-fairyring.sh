cd ../../fairyring
echo -e "\033[1m\033[43m*******************Starting the FairyRing...***************************\033[0m"
echo 1 | ./start-fairy.sh > fairylog.txt 2>&1 & sleep 5
echo -e "\033[1m\033[43m*******************Submitting the public key...***************************\033[0m"
./fairyringd tx keyshare create-latest-pub-key aac3880d4978a79f8c01630927e4b06020e45c6b7ded016a7df99d35a000fec24590d610a1c119d9a18d6b4270aad4f1 aac3880d4978a79f8c01630927e4b06020e45c6b7ded016a7df99d35a000fec24590d610a1c119d9a18d6b4270aad4f1 --from star -b sync 

cd ../fairybridge

# # ########
sleep 10
export RUST_LOG=debug
echo -e "\033[1m\033[43m*******************Running the relayer...***************************\033[0m"
cargo run --target x86_64-unknown-linux-gnu