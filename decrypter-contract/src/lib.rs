#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.

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
sol_interface! {
    interface IIBE {
        function decrypt(uint8[] memory private, uint8[] memory cv, uint8[] memory cw, uint8[] memory cu) external view returns (uint8[] memory);
    }
    interface IDecrypterChacha20 {
        function decrypter(uint8[] memory file_key, uint8[] memory nonce, uint8[] memory s) external pure returns (uint8[] memory);
    }
    interface IMacChacha20 {
        function headermac(uint8[] memory file_key, uint8[] memory body) external pure returns (uint8[] memory);
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
            Address::from_str("0x6d7190dbd5053b68687fc9406ff4b4075138f7f9").unwrap();
        let decrypter_contract: Address =
            Address::from_str("0x2494e4d946dd4423519fce5b68fdbdaf9afadd9d").unwrap();
        let mac_contract: Address =
            Address::from_str("0xf474512f901ece89fd15d0b23401077ee13666b2").unwrap();
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
    ibe_contract_addr: Address,
    decrypter_contract_addr: Address,
    mac_contract_addr: Address,
) -> Vec<u8> {
    // Parsing header and payload
    let (hdr, mut payload) = parse(src).unwrap();

    let file_key = unwrap(sk, &[*hdr.recipients[0].clone()], ibe_contract_addr);
    
   
    let mac_contract = IMacChacha20{address:mac_contract_addr};
    let mac = mac_contract.headermac(Call::new(),file_key.clone(),hdr.recipients[0].clone().body).unwrap();
   
    if mac.to_vec() != hdr.mac {
        return vec![];
    }
    let mut nonce = vec![0u8; 16];

    payload.read_exact(&mut nonce).unwrap(); // Handle potential errors properly

    let mut s: Vec<u8> = vec![0];
    let _ = payload.read_to_end(&mut s);
    // call new_reader or decrypter
   
    let decrypter_contract = IDecrypterChacha20{address:decrypter_contract_addr};
    let msg = decrypter_contract.decrypter(Call::new(),file_key.clone(),nonce,s).unwrap();
  
   
    msg
}

fn unwrap(sk: &G2Affine, stanzas: &[Stanza], ibe_contract: Address) -> Vec<u8>{
    // Check stanza length and type
    if stanzas.len() != 1 {
        return (vec![]);
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

fn unlock(signature: &G2Affine, ciphertext: &Ciphertext, ibe_contract_addr: Address) -> Vec<u8>{

    
    let ibe_contract = IIBE{address:ibe_contract_addr};
    let data = ibe_contract.decrypt(Call::new(),  signature.to_compressed().to_vec(),
    ciphertext.v.clone(),
    ciphertext.w.clone(),
    ciphertext.u.to_compressed().to_vec()).unwrap();
  
    data
}



