# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x6d7190dbd5053b68687fc9406ff4b4075138f7f9

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example ibe  "$address" "$sk"
