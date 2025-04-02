use std::sync::{Arc, RwLock};

use crate::{args::Grind, helper::cores::get_core_ids};
use anyhow::Result;

use spinners::{Spinner, Spinners};
use sui_keys::keystore::{AccountKeystore, InMemKeystore};
use sui_types::base_types::SuiAddress;

pub struct Keytool {}

impl Keytool {
    pub fn new() -> Self {
        Keytool {}
    }

    pub fn grind(&self, args: Grind) -> Result<(SuiAddress, String)> {
        let core_ids = get_core_ids(args.cores);
        let scheme = args.scheme;

        let catched = Arc::new(RwLock::new(false));

        let mut spinner = Spinner::new(
            Spinners::Dots8,
            format!("Grinding using {} cores...", core_ids.len()),
        );

        let handles: Vec<_> = core_ids
            .into_iter()
            .map(|i| {
                let args = args.clone();
                let catched = Arc::clone(&catched);

                std::thread::spawn(move || {
                    // Pin to core
                    let _ = core_affinity::set_for_current(i);

                    let mut key_store = InMemKeystore::default();

                    loop {
                        let ret = key_store.generate_and_add_new_key(scheme, None, None, None);
                        if ret.is_err() {
                            continue;
                        }

                        let (addr, s, _) = ret.unwrap();
                        if args.is_valid(&addr.to_string()) {
                            *catched.write().unwrap() = true;
                            return Some((addr, s));
                        }

                        // Check if we found a solution
                        if catched.read().unwrap().clone() {
                            return None;
                        }
                    }
                })
            })
            .collect();

        for h in handles {
            if let Ok(Some(solution)) = h.join() {
                return Ok(solution);
            }
        }

        spinner.stop();

        Err(anyhow::anyhow!("No solution found"))
    }
}
