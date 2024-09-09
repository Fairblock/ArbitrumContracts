#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use hmac::{Hmac, Mac, NewMac};
use serde::Deserialize;
use std::io::Write;
use std::io::{self, Read};
use std::str::FromStr;

use base64::{engine::general_purpose, write::EncoderWriter, Engine};
use hkdf::Hkdf;
use sha2::Sha256;
use stylus_sdk::prelude::*;
const INTRO: &str = "age-encryption.org/v1";

const RECIPIENT_PREFIX: &[u8] = b"->";

const FOOTER_PREFIX: &[u8] = b"---";

sol_storage! {
    #[entrypoint]
    pub struct MacChacha20 {

    }
}

#[external]

impl MacChacha20 {
    fn headermac(file_key: Vec<u8>, body: Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        if file_key.len() != 32 || body.is_empty() {
            return Err(stylus_sdk::call::Error::Revert(vec![1]));
        }

        let result = Stanza {
            type_: "distIBE".to_string(),
            args: vec![],
            body,
        };
        let hdr = Header {
            recipients: vec![Box::new(result)],
        };

        let h = Hkdf::<Sha256>::new(None, &file_key);
        let mut hmac_key = [0u8; 32];
        h.expand(b"header", &mut hmac_key)
            .map_err(|_| stylus_sdk::call::Error::Revert(vec![2]))?;

        let mut hh = Hmac::<Sha256>::new_from_slice(&hmac_key)
            .map_err(|_| stylus_sdk::call::Error::Revert(vec![3]))?;
        let mut hmac_writer = HmacWriter::new(hh.clone());

        hdr.marshal_without_mac(&mut hmac_writer)
            .map_err(|_| stylus_sdk::call::Error::Revert(vec![4]))?;

        hh = hmac_writer.0;
        Ok(hh.finalize().into_bytes().to_vec())
    }
}
#[derive(Clone, Deserialize)]
struct Stanza {
    type_: String,
    args: Vec<String>,
    body: Vec<u8>,
}
fn process_chunks_and_append(data: &[u8]) -> Vec<u8> {
    const CHUNK_SIZE: usize = 64;
    let mut result = Vec::new();

    for chunk in data.chunks(CHUNK_SIZE) {
        result.extend_from_slice(chunk);

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

        let new = process_chunks_and_append(enc);

        let mut ff: String = String::from_str("").unwrap();
        new.as_slice().read_to_string(&mut ff);
        write!(w, "{}", ff);

        let mut f: String = String::from_str("").unwrap();
        enc2.read_to_string(&mut f);
        write!(w, "{}", f);
        writeln!(w);

        w
    }
}

#[derive(Clone)]
struct Header {
    recipients: Vec<Box<Stanza>>,
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
