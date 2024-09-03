# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x6c81613befc3271dfd835a35eb79aac372409b88

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example pairing  "$address" "$sk"
