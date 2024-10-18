#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use hmac::{Hmac, Mac, NewMac};
use serde::Deserialize;
use std::io::Write;
use std::io::{self, Read};
use std::str::FromStr;
use base64::{engine::general_purpose, Engine};
use hkdf::Hkdf;
use sha2::Sha256;
use stylus_sdk::prelude::*;
const INTRO: &str = "age-encryption.org/v1";

sol_storage! {
    #[entrypoint]
    pub struct MacChacha20 {

    }
}

/// Calculates the header mac
///
/// # Parameters
///
/// - `key`: A `Vec<u8>` containing the symmetric key. It should be 32 bytes.
/// - `body`: A `Vec<u8>` containing the body of the ciphertext.

///
/// # Returns
///
/// - `Ok(Vec<u8>)`: If successful, returns a `Vec<u8>` containing the mac.
/// - `Err(stylus_sdk::call::Error)`: If an error occurs during decryption, it returns an error from the `stylus_sdk::call::Error` type.
#[public]
impl MacChacha20 {
    fn headermac(key: Vec<u8>, body: Vec<u8>) -> Result<Vec<u8>, stylus_sdk::call::Error> {
        if key.len() != 32 || body.is_empty() {
            return Err(stylus_sdk::call::Error::Revert("Wrong input length".as_bytes().to_vec()));
        }

        let result = Stanza {
            type_: "distIBE".to_string(),
            args: vec![],
            body,
        };
        let hdr = Header {
            recipients: vec![Box::new(result)],
        };

        let h = Hkdf::<Sha256>::new(None, &key);
        let mut hmac_key = [0u8; 32];
        h.expand(b"header", &mut hmac_key)
            .map_err(|_| stylus_sdk::call::Error::Revert("Key error".as_bytes().to_vec()))?;

        let mut hh = Hmac::<Sha256>::new_from_slice(&hmac_key)
            .map_err(|_| stylus_sdk::call::Error::Revert("Hash error".as_bytes().to_vec()))?;
        let mut hmac_writer = HmacWriter::new(hh.clone());

        hdr.marshal_without_mac(&mut hmac_writer)
            .map_err(|_| stylus_sdk::call::Error::Revert("Header error".as_bytes().to_vec()))?;

        hh = hmac_writer.0;
        Ok(hh.finalize().into_bytes().to_vec())
    }
}


fn process_chunks_and_append(data: &[u8]) -> Vec<u8> {
    const CHUNK_SIZE: usize = 64;
    let mut result = Vec::new();

    for chunk in data.chunks(CHUNK_SIZE) {
        result.extend_from_slice(chunk);

        if chunk.len() == 64 {
            result.push(10);
        }
    }

    result
}

#[derive(Clone, Deserialize)]
struct Stanza {
    type_: String,
    args: Vec<String>,
    body: Vec<u8>,
}

impl Stanza {
    fn marshal<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "->")?;
        write!(w, " {}", self.type_)?;
        for arg in &self.args {
            write!(w, " {}", arg)?;
        }
        writeln!(w)?;
        let encoded = general_purpose::STANDARD_NO_PAD.encode(&self.body);
        for chunk in encoded.as_bytes().chunks(64) {
            w.write_all(chunk)?;
            writeln!(w)?;
        }
        Ok(())
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
