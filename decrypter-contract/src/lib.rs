
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use base64::{engine::general_purpose, write::EncoderWriter, Engine};
use std::{io::Cursor, str::FromStr};
use bls12_381_plus::{G1Affine, G2Affine};
use serde::Deserialize;
use stylus_sdk::{
    alloy_primitives::Address,
    call::{call, Call},
  contract::address,
};
// use std::io::{Error, ErrorKind};
// mod ibe;


use hkdf::Hkdf;
use hmac::{Hmac, Mac, NewMac};
// use ibe::{decrypt, Ciphertext};
use sha2::Sha256;

use std::io::{self, BufRead, BufReader};
use std::io::{Read, Write};

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{prelude::sol_storage, prelude::sol_interface, stylus_proc::{entrypoint, external}};


sol_storage! {
    #[entrypoint]
    #[derive( Clone,Copy)]
    pub struct Decrypter {
       
    }
}
sol_interface! {
    interface IIBE {
        function decrypt(uint8[] memory private, uint8[] memory cv, uint8[] memory cw, uint8[] memory cu, address pairing_contract, address hasher_contract) external view returns (uint8[] memory);
    }
    interface IDecrypterChacha20 {
        function decrypter(uint8[] memory file_key, uint8[] memory nonce, uint8[] memory s) external pure returns (uint8[] memory);
    }
    interface IMacChacha20 {
        function headerMac(uint8[] memory file_key, string calldata type_, string[] memory args, uint8[] memory body) external pure returns (uint8[] memory);
    }
}

#[external]
impl Decrypter {
   
    pub fn decrypt(&self, c: Vec<u8>,  skbytes: Vec<u8>) -> core::result::Result<Vec<u8>, stylus_sdk::call::Error>{
        let sk = G2Affine::from_compressed(&skbytes.try_into().unwrap()).unwrap();
        let mut cursor = Cursor::new(c);
        let ibe_contract: Address = Address::from_str("0xE9d3Ad58d2d697B08B2ce777541Ddf30F1f060EC").unwrap();
        let decrypter_contract: Address =Address::from_str("0x438cc3c7E2Da22D897Ac8b5dc9509628B67EA13f").unwrap();
        let mac_contract: Address =Address::from_str("0x73c90f1B5c1DE9c73e4c68E6e1D4Ad7E48C5a7Fc").unwrap();
        let decrypted = Decrypt(*self,&sk, &mut cursor, IIBE { address: ibe_contract },IDecrypterChacha20 { address: decrypter_contract },IMacChacha20 { address: mac_contract });
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

fn is_valid_string(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars().all(|c| (33..=126).contains(&(c as u32)))
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
            if r.is_some() {}
            let (prefix, args) = split_args(&line.as_bytes());
            if prefix.as_bytes() != FOOTER_PREFIX || args.len() != 1 {}
            h.mac = decode_string(&args[0]); // Assuming decode_string is defined
            break;
        } else if line.as_bytes().starts_with(RECIPIENT_PREFIX) {
            if r.is_some() {}
            r = Some(Stanza {
                type_: String::new(),
                args: Vec::new(),
                body: Vec::new(),
            });
            let (prefix, args) = split_args(&line.as_bytes());

            if prefix.as_bytes() != RECIPIENT_PREFIX || args.is_empty() {}
            if args.iter().any(|a| !is_valid_string(a)) {}

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


struct HmacWriter(Hmac<Sha256>);

impl HmacWriter {
    fn new(hmac: Hmac<Sha256>) -> Self {
        HmacWriter(hmac)
    }
}

impl Write for HmacWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
// Additional helper functions and modules (like 'armor' and 'parse') would be needed based on the actual 'ibe' package

// fn header_mac(file_key: &[u8], hdr: &Header) -> Vec<u8> {
//     let h = Hkdf::<Sha256>::new(None, file_key);
//     let mut hmac_key = [0u8; 32];
//     let _ = h.expand(b"header", &mut hmac_key);

//     let hh = Hmac::<Sha256>::new_from_slice(&hmac_key).expect("HMAC can take key of any size");
//     let mut hmac_writer = HmacWriter::new(hh.clone());
//     let _ =  hdr.marshal_without_mac(&mut hmac_writer);

//     hh.finalize().into_bytes().to_vec()
// }

#[derive( Clone, Deserialize)]
struct Stanza {
    type_: String, // 'type' is a reserved keyword in Rust, so we use 'type_'
    args: Vec<String>,
    body: Vec<u8>,
}

pub fn Decrypt<'a>(dec : Decrypter, sk: &G2Affine, src: &'a mut dyn Read,ibe_contract : IIBE, decrypter_contract : IDecrypterChacha20, mac_contract:IMacChacha20) -> Vec<u8> {
    // Parsing header and payload
    let (hdr, mut payload) = parse(src).unwrap();

    // let mut stanzas: Vec<Stanza> = Vec::with_capacity(hdr.recipients.len());
    // for s in &hdr.recipients {
    //     stanzas.push(*s.clone()); // Assuming Stanza can be cloned
    // }

    // let stanzas_slice = &stanzas[..];

    let file_key = unwrap(dec, sk, &[*hdr.recipients[0].clone()],ibe_contract);

    let mac = mac_contract.header_mac(&dec,file_key.clone(),hdr.recipients[0].clone().type_,hdr.recipients[0].clone().args,hdr.recipients[0].clone().body).unwrap();
   
    if mac != hdr.mac {
         return vec![];
    }
    let mut nonce = vec![0u8; 16];

    payload.read_exact(&mut nonce).unwrap(); // Handle potential errors properly
     //todo
                                             // Creating a decrypted data stream
                                             //
    let mut s: Vec<u8> = vec![0];
    let _ = payload.read_to_end(&mut s);
    // call new_reader or decrypter
    let msg = decrypter_contract.decrypter(&dec, file_key, nonce, s).unwrap();
    msg
}

fn unwrap(dec: Decrypter,sk: &G2Affine, stanzas: &[Stanza],ibe_contract : IIBE) -> Vec<u8> {
    // Check stanza length and type
    if stanzas.len() != 1 {
        return (vec![0]);
    }

    // Convert bytes to ciphertext and perform the unlock operation
    let ciphertext = bytes_to_ciphertext(&stanzas[0].body);
   
    (unlock(dec, sk, &ciphertext, ibe_contract))
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
        return Ciphertext {
            u: todo!(),
            v: todo!(),
            w: todo!(),
        };
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

fn unlock(
    dec: Decrypter,
    signature: &G2Affine,
    ciphertext: &Ciphertext,
    ibe_contract: IIBE
) -> Vec<u8> {
    //todo
   // let return_data = call(Call::new_in(storage), Address::default(), &[0u8]).unwrap();
    //    let data = decrypt(*signature, ciphertext);
   // function decrypt(uint8[96] calldata private, uint8[] memory cv, uint8[] memory cw, uint8[] memory cu, address pairing_contract, address hasher_contract) external view returns (uint8[] memory);
   let pairing_contract_addr: Address = Address::from_str("0x3cBf6b597De34F1559abFb16FD7Fe744d6b89713").unwrap();
   let hasher_contract_addr: Address =Address::from_str("0x5414210Fe884C561D3AF2c92D71166282d7f50fb").unwrap();
    let data = ibe_contract.decrypt(&dec,signature.to_compressed().to_vec(),ciphertext.v.clone(),ciphertext.w.clone(),ciphertext.u.to_compressed().to_vec(),pairing_contract_addr, hasher_contract_addr).unwrap();
    data
}

