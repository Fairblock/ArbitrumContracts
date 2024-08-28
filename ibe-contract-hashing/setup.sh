# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x2c397820261d13080e404b4ff25aa0e16e2062b2

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example hashing  "$address" "$sk"
