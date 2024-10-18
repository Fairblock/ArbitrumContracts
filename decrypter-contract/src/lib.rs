#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use base64::{engine::general_purpose, Engine};
use ic_bls12_381::{pairing, G1Affine, G2Affine};
use serde::Deserialize;
use std::io::Read;
use std::io::{self, BufRead, BufReader};
use std::{io::Cursor, str::FromStr};
use stylus_sdk::alloy_primitives::Address;
use stylus_sdk::alloy_sol_types;
use stylus_sdk::call::Call;
use stylus_sdk::prelude::public;
use stylus_sdk::{
    prelude::{sol_interface, sol_storage},
    stylus_proc::entrypoint,
};

const INTRO: &str = "age-encryption.org/v1";
const RECIPIENT_PREFIX: &[u8] = b"->";
const FOOTER_PREFIX: &[u8] = b"---";
const COLUMNS_PER_LINE: usize = 64;
const BYTES_PER_LINE: usize = COLUMNS_PER_LINE / 4 * 3;
const KYBER_POINT_LEN: usize = 48;
const CIPHER_V_LEN: usize = 32;
const CIPHER_W_LEN: usize = 32;

pub struct Ciphertext {
    pub u: G1Affine,
    pub v: Vec<u8>,
    pub w: Vec<u8>,
}

struct Header {
    recipients: Vec<Box<Stanza>>,
    mac: Vec<u8>,
}
#[derive(Clone, Deserialize)]
struct Stanza {
    type_: String,
    args: Vec<String>,
    body: Vec<u8>,
}

sol_storage! {
    #[entrypoint]
    #[derive( Clone,Copy)]
    pub struct Decrypter {

    }
}
sol_interface! {
    interface IIBE {
        function decrypt(uint8[] memory r_gid, uint8[] memory cv, uint8[] memory cw, uint8[] memory cu) external view returns (uint8[] memory);
    }
    interface IDecrypterChacha20 {
        function decrypter(uint8[] memory key, uint8[] memory nonce, uint8[] memory ciphertext) external pure returns (uint8[] memory);
    }
    interface IMacChacha20 {
        function headermac(uint8[] memory key, uint8[] memory body) external pure returns (uint8[] memory);
    }
}


/// Performs the decryption.
///
/// # Parameters
///
/// - `c`: A `Vec<u8>` representing the ciphertext to be decrypted.
/// - `skbytes`: A `Vec<u8>` containing the decryption key bytes.
///
/// # Returns
///
/// - `Ok(Vec<u8>)`: If successful, returns a `Vec<u8>` containing the plaintext.
/// - `Err(stylus_sdk::call::Error)`: If an error occurs during decryption, it returns an error from the `stylus_sdk::call::Error` type.
#[public]
impl Decrypter {
    pub fn decrypt(
        &self,
        c: Vec<u8>,
        skbytes: Vec<u8>,
    ) -> core::result::Result<Vec<u8>, stylus_sdk::call::Error> {
        if skbytes.len() != 96 {
            return Err(stylus_sdk::call::Error::Revert(
                "Invalid compressed G2Affine length".as_bytes().to_vec(),
            ));
        }
        let sk_ct_option = G2Affine::from_compressed(&skbytes.try_into().unwrap());
        if sk_ct_option.is_none().into() {
            return Err(stylus_sdk::call::Error::Revert(
                "Invalid compressed G2Affine".as_bytes().to_vec(),
            ));
        }
        let sk = sk_ct_option.unwrap();

        let mut cursor = Cursor::new(c);

        let ibe_contract = Address::from_str("0x61aabc5cad6babbccc3aff0484fa46435869dcc4")
            .map_err(|_| {
                return stylus_sdk::call::Error::Revert("Invalid ibe_contract address".as_bytes().to_vec())
            })?;

        let decrypter_contract = Address::from_str("0x9b93929e458dd949ef9e26743aa2465a2c8b4c13")
            .map_err(|_| {
               return stylus_sdk::call::Error::Revert(
                    "Invalid decrypter_contract address".as_bytes().to_vec(),
                )
            })?;

        let mac_contract = Address::from_str("0x6df2dbf827c938f3276b09a2a096be6249c2e996")
            .map_err(|_| {
               return stylus_sdk::call::Error::Revert("Invalid mac_contract address".as_bytes().to_vec())
            })?;

        let decrypted = decrypter(
            &sk,
            &mut cursor,
            ibe_contract,
            decrypter_contract,
            mac_contract,
        );

        decrypted
    }
}

pub fn decrypter<'a>(
    sk: &G2Affine,
    src: &'a mut dyn Read,
    ibe_contract_addr: Address,
    decrypter_contract_addr: Address,
    mac_contract_addr: Address,
) -> core::result::Result<Vec<u8>, stylus_sdk::call::Error> {
    let (hdr, mut payload) = parse(src).unwrap();

    let file_key = unwrap(sk, &[*hdr.recipients[0].clone()], ibe_contract_addr)?;
 
    let mac_contract = IMacChacha20 {
        address: mac_contract_addr,
    };
    let mac = mac_contract
        .headermac(
            Call::new(),
            file_key.clone(),
            hdr.recipients[0].clone().body,
        )
        .map_err(|_| stylus_sdk::call::Error::Revert("MAC contract error".as_bytes().to_vec()))?;

    if mac.to_vec() != hdr.mac {
        return Err(stylus_sdk::call::Error::Revert(
            "MACs not matching".as_bytes().to_vec(),
        ));
    }
    let mut nonce = vec![0u8; 16];

    let _ = payload
        .read_exact(&mut nonce)
        .map_err(|_| stylus_sdk::call::Error::Revert("Payload reading error".as_bytes().to_vec()))?;

    let mut ciphertext: Vec<u8> = vec![];
    let output = payload.read_to_end(&mut ciphertext);
    if output.is_err(){
        return Err(stylus_sdk::call::Error::Revert(
            "Payload reading error".as_bytes().to_vec(),
        ));
    }
    let decrypter_contract = IDecrypterChacha20 {
        address: decrypter_contract_addr,
    };
    let msg = decrypter_contract
        .decrypter(Call::new(), file_key.clone(), nonce, ciphertext)
        .map_err(|_| {
            stylus_sdk::call::Error::Revert(
                "Chacha20 decryption contract error".as_bytes().to_vec(),
            )
        });

    msg
}

fn unwrap(sk: &G2Affine, stanzas: &[Stanza], ibe_contract: Address) -> core::result::Result<Vec<u8>, stylus_sdk::call::Error> {
    if stanzas.len() != 1 {
        return Err(stylus_sdk::call::Error::Revert(
            "Chacha20 decryption contract error".as_bytes().to_vec()
        ));
    }
    let ciphertext = bytes_to_ciphertext(&stanzas[0].body);
    unlock(sk, &ciphertext, ibe_contract)
}

fn unlock(signature: &G2Affine, ciphertext: &Ciphertext, ibe_contract_addr: Address) -> core::result::Result<Vec<u8>, stylus_sdk::call::Error>{
    let r_gid = pairing(&ciphertext.u, signature);

    let ibe_contract = IIBE {
        address: ibe_contract_addr,
    };
    let data = ibe_contract
        .decrypt(
            Call::new(),
            r_gid.to_bytes().to_vec(),
            ciphertext.v.clone(),
            ciphertext.w.clone(),
            ciphertext.u.to_compressed().to_vec(),
        );

    data
}

fn bytes_to_ciphertext(b: &[u8]) -> Ciphertext {
    let exp_len = KYBER_POINT_LEN + CIPHER_V_LEN + CIPHER_W_LEN;
    if b.len() != exp_len {
        return Ciphertext{u:G1Affine::default(),v:vec![],w:vec![]};
    }

    let kyber_point = &b[0..KYBER_POINT_LEN];
    let cipher_v = &b[KYBER_POINT_LEN..KYBER_POINT_LEN + CIPHER_V_LEN];
    let cipher_w = &b[KYBER_POINT_LEN + CIPHER_V_LEN..];

    let u: G1Affine = G1Affine::from_compressed(convert_slice_to_array(kyber_point)).unwrap();

    let ct = Ciphertext {
        u,
        v: cipher_v.to_vec(),
        w: cipher_w.to_vec(),
    };

    ct
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
    let decoded = general_purpose::STANDARD_NO_PAD.decode(s);
    if decoded.is_err() {
        return vec![];
    }
    return decoded.unwrap();
}

fn convert_slice_to_array(slice: &[u8]) -> &[u8; 48] {
    if slice.len() != 48 {
        return &[0u8; 48];
    }
    let array_ref: &[u8; 48] = slice.try_into().map_err(|_| "Failed to convert").unwrap();
    array_ref
}


fn parse<'a, R: Read + 'a>(input: R) -> io::Result<(Header, Box<dyn Read + 'a>)> {
    let mut rr = BufReader::new(input);
    let mut line = String::new();

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
        }

        let line = String::from_utf8_lossy(&line_bytes).into_owned();

        if line.as_bytes().starts_with(FOOTER_PREFIX) {
            let (prefix, args) = split_args(&line.as_bytes());
            if prefix.as_bytes() != FOOTER_PREFIX || args.len() != 1 {}
            h.mac = decode_string(&args[0]);
            break;
        } else if line.as_bytes().starts_with(RECIPIENT_PREFIX) {
            r = Some(Stanza {
                type_: String::new(),
                args: Vec::new(),
                body: Vec::new(),
            });
            let (_, args) = split_args(&line.as_bytes());

            let stanza = r.as_mut().unwrap();
            stanza.type_ = args[0].clone();
            stanza.args = args[1..].to_vec();

            h.recipients.push(Box::new(stanza.clone()));
        } else if let Some(_stanza) = r.as_mut() {
            let b = decode_string(&line.trim_end());
            if b.len() > BYTES_PER_LINE {}
            h.recipients[0].body.extend_from_slice(&b);

            if b.len() < BYTES_PER_LINE {
                r = None;
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

