
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use serde::Deserialize;
use std::io::{self, Read};
use std::io::{ Write};
use std::str::FromStr;
use hmac::{Hmac, Mac, NewMac};

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::prelude::*;
use base64::{engine::general_purpose, write::EncoderWriter, Engine};
use hkdf::Hkdf;
use sha2::Sha256;
const INTRO: &str = "age-encryption.org/v1";

const RECIPIENT_PREFIX: &[u8] = b"->";

const FOOTER_PREFIX: &[u8] = b"---";

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct MacChacha20 {
       
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]

impl MacChacha20 {
   

    
    fn header_mac(file_key: Vec<u8>, type_ :String, args: Vec<String>, body: Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error> {
       // let result: Stanza = serde_json::from_slice(r.as_slice()).expect("Deserialization failed");
       let result = Stanza{type_:type_,args:args,body:body};
        let hdr = Header{recipients:vec![Box::new(result)]};
        let h = Hkdf::<Sha256>::new(None, file_key.as_slice());
        let mut hmac_key = [0u8; 32];
        h.expand(b"header", &mut hmac_key);
        let mut hh = Hmac::<Sha256>::new_from_slice(&hmac_key).expect("HMAC can take key of any size");
        let mut hmac_writer = HmacWriter::new(hh.clone());
        hdr.marshal_without_mac(&mut hmac_writer);
        hh = hmac_writer.0;
        Ok(hh.finalize().into_bytes().to_vec())
    }
    

}
#[derive( Clone, Deserialize)]
struct Stanza {
    type_: String, // 'type' is a reserved keyword in Rust, so we use 'type_'
    args: Vec<String>,
    body: Vec<u8>,
}
fn process_chunks_and_append(data: &[u8]) -> Vec<u8> {
    const CHUNK_SIZE: usize = 64;
    let mut result = Vec::new();

    for chunk in data.chunks(CHUNK_SIZE) {
        // Append the chunk to the result vector.
        result.extend_from_slice(chunk);
        // Append [10] after the chunk.
        if (chunk.len() == 64) {
            result.push(10);
        }
    }

    result
}
impl Stanza {
    fn marshal<'a, W: Write>(&'a self, w: &'a mut W) -> &mut W {
        write!(w, "{}", "->");

        write!(w, " {}", self.type_);

        for arg in &self.args {
            write!(w, " {}", arg);
        }
        writeln!(w);
        let b = self.body.clone();
        let encoded: String = general_purpose::STANDARD_NO_PAD.encode(b.as_slice());
        let l = encoded.as_bytes().len() - 2;
        let enc = &encoded.as_bytes()[..l];
        let mut enc2 = &encoded.as_bytes()[l..];
        // panic!("{:?}", encoded.as_bytes());
        let new = process_chunks_and_append(enc);

        let mut ff: String = String::from_str("").unwrap();
        new.as_slice().read_to_string(&mut ff);
        write!(w, "{}", ff);

        let mut f: String = String::from_str("").unwrap();
        enc2.read_to_string(&mut f);
        write!(w, "{}", f);
        writeln!(w);
        // writeln!(w);
        w
    }
}

#[derive( Clone)]
struct Header {
    recipients: Vec<Box<Stanza>>, // Vec of boxed (heap-allocated) Stanza objects
             // Vec<u8> is equivalent to a slice of bytes ([]byte in Go)
}
impl Header {
    fn marshal_without_mac<W: Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "{}", INTRO)?;
        for r in &self.recipients {
            r.marshal(w);
        }
        write!(w, "{}", "---")
    }
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