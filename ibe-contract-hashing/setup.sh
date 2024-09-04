# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x9ff55191d27f5e311549687527dabb44096253c5

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example hashing  "$address" "$sk"
