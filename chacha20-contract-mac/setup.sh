# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x4e41d71024260f923a7e53f7f20d5c375da5f7f0

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example mac  "$address" "$sk"
