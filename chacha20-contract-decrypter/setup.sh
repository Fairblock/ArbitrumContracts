# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x91d976ef94a1b2bb6c24097f335037342f0b031e

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example decrypter  "$address" "$sk"
