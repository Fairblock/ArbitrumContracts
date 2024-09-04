# read -p "Enter the wallet private key: " sk
sk=b327fa89f163dd2b3939385fe1190ce86e790f33d320f1512da5b481ec1ca1af

address=0x54410c800e89f30d46a745f6f6809967fe6d049d

RUST_BACKTRACE=full cargo +nightly-2024-05-20 run --example decrypter  "$address" "$sk"
