# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x546173b6a05b305ca395706da1b7c85c812e3e30

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example ibe  "$address" "$sk"
