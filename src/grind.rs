use crate::args::Grind;
use anyhow::Result;

use sui_keys::keystore::{AccountKeystore, InMemKeystore};
use sui_types::{base_types::SuiAddress, crypto::SignatureScheme};

pub struct Keytool {}

impl Keytool {
    pub fn new() -> Self {
        Keytool {}
    }

    pub fn grind(&self, args: Grind) -> Result<(SuiAddress, String)> {
        let mut i = 0;
        let key_scheme = args.scheme;
        loop {
            let (addr, s) = self.generate(key_scheme)?;
            if args.is_valid(&addr.to_string()) {
                return Ok((addr, s));
            }
            i += 1;
            println!("\rInteration: {}", i);
        }
    }

    pub fn generate(&self, scheme: SignatureScheme) -> Result<(SuiAddress, String)> {
        let mut key_store = InMemKeystore::default();
        let (addr, sp, _) = key_store.generate_and_add_new_key(scheme, None, None, None)?;
        Ok((addr, sp))
    }
}
