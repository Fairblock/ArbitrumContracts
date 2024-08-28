#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use base64::{engine::general_purpose, Engine};
use bls12_381_plus::{G1Affine, G2Affine};
use ethabi::Token;
use serde::Deserialize;
use std::{io::Cursor, str::FromStr};
use stylus_sdk::alloy_primitives::Address;
use stylus_sdk::alloy_sol_types;
use stylus_sdk::{
    call::call,
    prelude::{sol_interface, sol_storage},
    stylus_proc::{entrypoint, external},
};

use stylus_sdk::call::Call;

// use ibe::{decrypt, Ciphertext};
use sha2::Sha256;

use std::io::{self, BufRead, BufReader};
use std::io::{Read, Write};

sol_storage! {
    #[entrypoint]
    #[derive( Clone,Copy)]
    pub struct Decrypter {

    }
}

#[external]
impl Decrypter {
    pub fn decrypt(
        &self,
        c: Vec<u8>,
        skbytes: Vec<u8>,
    ) -> core::result::Result<Vec<u8>, stylus_sdk::call::Error> {
        let sk = G2Affine::from_compressed(&skbytes.try_into().unwrap()).unwrap();

        let mut cursor = Cursor::new(c);
        let ibe_contract: Address =
            Address::from_str("0x6fe53df98f956841199f3aa4b89bdd4f23bc4783").unwrap();
        let decrypter_contract: Address =
            Address::from_str("0x91d976ef94a1b2bb6c24097f335037342f0b031e").unwrap();
        let mac_contract: Address =
            Address::from_str("0x4e41d71024260f923a7e53f7f20d5c375da5f7f0").unwrap();
        let decrypted = Decrypt(
            &sk,
            &mut cursor,
            ibe_contract,
            decrypter_contract,
            mac_contract,
        );
        Ok(decrypted)
    }
}

// Constants
const INTRO: &str = "age-encryption.org/v1";

const RECIPIENT_PREFIX: &[u8] = b"->";

const FOOTER_PREFIX: &[u8] = b"---";

const COLUMNS_PER_LINE: usize = 64;
const BYTES_PER_LINE: usize = COLUMNS_PER_LINE / 4 * 3;

const kyber_point_len: usize = 48;
const cipher_v_len: usize = 32;
const cipher_w_len: usize = 32;

pub struct Ciphertext {
    pub u: G1Affine,
    pub v: Vec<u8>,
    pub w: Vec<u8>,
}

struct Header {
    recipients: Vec<Box<Stanza>>, // Vec of boxed (heap-allocated) Stanza objects
    mac: Vec<u8>,                 // Vec<u8> is equivalent to a slice of bytes ([]byte in Go)
}

fn split_args(line: &[u8]) -> (String, Vec<String>) {
    let line_str = String::from_utf8_lossy(line);
    let trimmed_line = line_str.trim_end_matches('\n');
    let parts: Vec<String> = trimmed_line.split_whitespace().map(String::from).collect();

    if !parts.is_empty() {
        (parts[0].clone(), parts[1..].to_vec())
    } else {
        (String::new(), Vec::new())
    }
}

fn decode_string(s: &str) -> Vec<u8> {
    general_purpose::STANDARD_NO_PAD.decode(s).unwrap()
}

fn parse<'a, R: Read + 'a>(input: R) -> io::Result<(Header, Box<dyn Read + 'a>)> {
    let mut rr = BufReader::new(input);
    let mut line = String::new();

    // Read the intro line
    rr.read_line(&mut line)?;
    if line.trim_end() != INTRO {}

    let mut h = Header {
        recipients: Vec::new(),
        mac: Vec::new(),
    };
    let mut r: Option<Stanza> = None;

    loop {
        let mut line_bytes = Vec::new();
        let bytes_read = rr.read_until(b'\n', &mut line_bytes)?;
        if bytes_read == 0 {
            break;
        } // End of file or error

        let line = String::from_utf8_lossy(&line_bytes).into_owned();

        if line.as_bytes().starts_with(FOOTER_PREFIX) {
            let (prefix, args) = split_args(&line.as_bytes());
            if prefix.as_bytes() != FOOTER_PREFIX || args.len() != 1 {}
            h.mac = decode_string(&args[0]); // Assuming decode_string is defined
            break;
        } else if line.as_bytes().starts_with(RECIPIENT_PREFIX) {
            r = Some(Stanza {
                type_: String::new(),
                args: Vec::new(),
                body: Vec::new(),
            });
            let (prefix, args) = split_args(&line.as_bytes());

            let stanza = r.as_mut().unwrap();
            stanza.type_ = args[0].clone();
            stanza.args = args[1..].to_vec();

            h.recipients.push(Box::new(stanza.clone()));
        } else if let Some(stanza) = r.as_mut() {
            let b = decode_string(&line.trim_end());
            if b.len() > BYTES_PER_LINE {}
            h.recipients[0].body.extend_from_slice(&b);

            if b.len() < BYTES_PER_LINE {
                r = None; // Only the last line of a body can be short
            }
        } else {
        }
    }

    let payload = if rr.buffer().is_empty() {
        Box::new(rr.into_inner()) as Box<dyn Read>
    } else {
        let buffer = rr.buffer().to_vec();
        let remaining_input = rr.into_inner();
        Box::new(io::Cursor::new(buffer).chain(remaining_input)) as Box<dyn Read>
    };

    Ok((h, payload))
}

#[derive(Clone, Deserialize)]
struct Stanza {
    type_: String, // 'type' is a reserved keyword in Rust, so we use 'type_'
    args: Vec<String>,
    body: Vec<u8>,
}

pub fn Decrypt<'a>(
    sk: &G2Affine,
    src: &'a mut dyn Read,
    ibe_contract: Address,
    decrypter_contract: Address,
    mac_contract: Address,
) -> Vec<u8> {
    // Parsing header and payload
    let (hdr, mut payload) = parse(src).unwrap();

    let file_key = unwrap(sk, &[*hdr.recipients[0].clone()], ibe_contract);
    return file_key;
    let calldata = encode_function_header_mac(
        file_key.clone(),
        hdr.recipients[0].clone().type_,
        hdr.recipients[0].clone().args,
        hdr.recipients[0].clone().body,
    );
    let mac = match call(Call::new(), mac_contract, &calldata) {
        Ok(value) => value,
        Err(e) => {
            return vec![];
        }
    };
    let mac_data= mac[64..64+mac[63] as usize].to_vec();
    if mac_data != hdr.mac {
        return vec![];
    }
    let mut nonce = vec![0u8; 16];

    payload.read_exact(&mut nonce).unwrap(); // Handle potential errors properly

    let mut s: Vec<u8> = vec![0];
    let _ = payload.read_to_end(&mut s);
    // call new_reader or decrypter
   
    
    let calldata = encode_function_decrypter(file_key, nonce, s);
    let msg = match call(Call::new(), decrypter_contract, &calldata) {
        Ok(value) => value,
        Err(e) => {
            return vec![];
        }
    };
    let msg_data= msg[64..64+msg[63] as usize].to_vec();
    msg_data
}

fn unwrap(sk: &G2Affine, stanzas: &[Stanza], ibe_contract: Address) -> Vec<u8> {
    // Check stanza length and type
    if stanzas.len() != 1 {
        return (vec![0]);
    }

    // Convert bytes to ciphertext and perform the unlock operation
    let ciphertext = bytes_to_ciphertext(&stanzas[0].body);

    (unlock(sk, &ciphertext, ibe_contract))
}

fn convert_slice_to_array(slice: &[u8]) -> &[u8; 48] {
    if slice.len() != 48 {
        return &[0u8; 48];
    }

    let array_ref: &[u8; 48] = slice.try_into().map_err(|_| "Failed to convert").unwrap();
    array_ref
}

// The Rust function
fn bytes_to_ciphertext(b: &[u8]) -> Ciphertext {
    let exp_len = kyber_point_len + cipher_v_len + cipher_w_len;
    if b.len() != exp_len {
        panic!("error");
    }

    let kyber_point = &b[0..kyber_point_len];
    let cipher_v = &b[kyber_point_len..kyber_point_len + cipher_v_len];
    let cipher_w = &b[kyber_point_len + cipher_v_len..];

    let u: G1Affine = G1Affine::from_compressed(convert_slice_to_array(kyber_point)).unwrap();

    let ct = Ciphertext {
        u,
        v: cipher_v.to_vec(),
        w: cipher_w.to_vec(),
    };

    ct
}

fn unlock(signature: &G2Affine, ciphertext: &Ciphertext, ibe_contract: Address) -> Vec<u8> {
    let pairing_contract_addr: String = "0x13544f0d527f74706b862ae87f2b13b89ee1d190".to_string();
    let hasher_contract_addr: String = "0x2c397820261d13080e404b4ff25aa0e16e2062b2".to_string();
    let calldata = encode_function_decrypt(
        signature.to_compressed().to_vec(),
        ciphertext.v.clone(),
        ciphertext.w.clone(),
        ciphertext.u.to_compressed().to_vec(),
        pairing_contract_addr,
        hasher_contract_addr,
    );
    let mut v = signature.to_compressed().to_vec();
    
    return v;
    let data = match call(Call::new(), ibe_contract, &calldata) {
        Ok(value) => value,
        Err(e) => {
            return vec![];
        }
    };
    let data_data= data[64..64+data[63] as usize].to_vec();
    data_data
}

fn encode_function_header_mac(
    file_key: Vec<u8>,
    type_: String,
    args: Vec<String>,
    body: Vec<u8>,
) -> Vec<u8> {
    let function_signature: [u8; 4] = [0xa9, 0x1e, 0x5a, 0xd3];
    // Prepare the inputs as Tokens
    let mut args_token: Vec<Token> = Vec::new();
    let mut i = 0;
    for item in args {
        args_token.insert(i, Token::String(item));
        i = i + 1;
    }
    let inputs: Vec<Token> = vec![
        Token::Bytes(file_key),
        Token::String(type_),
        Token::Array(args_token),
        Token::Bytes(body),
    ];

    // Encode the inputs with the function signature
    let mut encoded = function_signature.to_vec();
    let encoded_params = ethabi::encode(&inputs);

    // Combine the function signature with the encoded parameters
    encoded.extend_from_slice(&encoded_params);

    encoded
}
fn encode_function_decrypter(file_key: Vec<u8>, nonce: Vec<u8>, s: Vec<u8>) -> Vec<u8> {
    let function_signature: [u8; 4] = [0x50, 0xdc, 0x7e, 0xb1];
    // Prepare the inputs as Tokens
    let inputs: Vec<Token> = vec![Token::Bytes(file_key), Token::Bytes(nonce), Token::Bytes(s)];

    // Encode the inputs with the function signature
    let mut encoded = function_signature.to_vec();
    let encoded_params = ethabi::encode(&inputs);

    // Combine the function signature with the encoded parameters
    encoded.extend_from_slice(&encoded_params);

    encoded
}

fn encode_function_decrypt(
    private: Vec<u8>,
    cv: Vec<u8>,
    cw: Vec<u8>,
    cu: Vec<u8>,
    pairing_contract: String,
    hasher_contract: String,
) -> Vec<u8> {
    let function_signature: [u8; 4] = [0x39, 0x83, 0xd4, 0x30];
    // Prepare the inputs as Tokens
    let inputs: Vec<Token> = vec![
        Token::Bytes(private),
        Token::Bytes(cv),
        Token::Bytes(cw),
        Token::Bytes(cu),
        Token::String(pairing_contract),
        Token::String(hasher_contract),
    ];

    // Encode the inputs with the function signature
    let mut encoded = function_signature.to_vec();
    let encoded_params = ethabi::encode(&inputs);

    // Combine the function signature with the encoded parameters
    encoded.extend_from_slice(&encoded_params);

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use bls12_381_plus::{G1Affine, G2Affine};

    #[test]
    fn test_decrypt() {
        // Example compressed private key and ciphertext (for testing purposes only)
        let skbytes = vec![180, 94, 231, 64, 60, 139, 63, 77, 251, 219, 173, 163, 74, 124, 6, 10, 129, 139, 151, 186, 102, 134, 86, 99, 150, 127, 59, 169, 18, 212, 67, 132, 48, 180, 58, 172, 181, 219, 30, 166, 33, 104, 186, 198, 23, 29, 20, 141, 15, 107, 179, 56, 147, 33, 220, 105, 191, 20, 32, 206, 3, 203, 206, 179, 228, 207, 247, 100, 37, 47, 155, 29, 212, 118, 240, 159, 79, 249, 88, 182, 208, 106, 20, 154, 236, 61, 92, 86, 122, 253, 31, 5, 161, 65, 125, 200];
        let c = vec![97, 103, 101, 45, 101, 110, 99, 114, 121, 112, 116, 105, 111, 110, 46, 111, 114, 103, 47, 118, 49, 10, 45, 62, 32, 100, 105, 115, 116, 73, 66, 69, 10, 115, 56, 109, 66, 120, 119, 65, 76, 78, 88, 100, 76, 102, 51, 122, 98, 98, 119, 113, 84, 90, 81, 116, 65, 66, 54, 109, 51, 115, 67, 52, 57, 121, 71, 67, 54, 43, 50, 119, 103, 121, 108, 66, 81, 71, 101, 121, 50, 56, 52, 69, 82, 79, 82, 76, 70, 81, 84, 71, 75, 114, 88, 56, 53, 10, 52, 106, 70, 85, 57, 86, 103, 87, 65, 79, 50, 88, 82, 105, 48, 80, 102, 83, 50, 121, 47, 71, 107, 55, 52, 101, 56, 104, 55, 83, 73, 54, 115, 47, 54, 77, 86, 115, 114, 48, 114, 87, 106, 103, 117, 97, 77, 100, 99, 80, 80, 57, 121, 110, 54, 73, 118, 116, 109, 68, 65, 110, 43, 115, 10, 98, 51, 85, 53, 101, 56, 118, 51, 73, 76, 52, 118, 74, 67, 57, 82, 49, 108, 54, 103, 50, 65, 10, 45, 45, 45, 32, 99, 104, 104, 109, 106, 51, 70, 66, 51, 99, 79, 121, 71, 83, 103, 57, 101, 72, 48, 104, 112, 77, 117, 81, 114, 97, 98, 79, 98, 85, 118, 119, 101, 55, 88, 101, 80, 67, 56, 55, 112, 52, 69];

        // Create an instance of Decrypter
        let decrypter = Decrypter {};

        // Call the decrypt function
        let result = decrypter.decrypt(c, skbytes);

        // Ensure the result is Ok and the decrypted data matches the expected output
        match result {
            Ok(decrypted_data) => {
                // You can assert against expected data here
                // assert_eq!(decrypted_data, expected_data);
                println!("Decrypted data: {:?}", decrypted_data);
            }
            Err(e) => {
                panic!("Decryption failed: {:?}", e);
            }
        }
    }
}