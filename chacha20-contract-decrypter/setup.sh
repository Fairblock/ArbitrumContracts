# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x60357b4d18142a442e62df7d0d78fd7db229aaea

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example decrypter  "$address" "$sk"
