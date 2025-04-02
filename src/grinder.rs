use std::sync::{Arc, RwLock};

use crate::{args::GrindArgs, helper::cores::get_core_ids};
use anyhow::Result;

use spinners::{Spinner, Spinners};
use sui_keys::keystore::{AccountKeystore, InMemKeystore};
use sui_types::base_types::SuiAddress;

pub struct Grinder {}

impl Grinder {
    pub fn new() -> Self {
        Grinder {}
    }

    /// Grind the keypair using the given arguments
    /// Returns the address and seedphrase if a solution is found
    /// If no solution is found, it returns an error
    /// The grinding process will be done in parallel using the specified number of cores
    /// If no cores are specified, it will use all available cores
    pub fn grind(&self, args: GrindArgs) -> Result<(SuiAddress, String)> {
        let core_ids = get_core_ids(args.cores);
        let scheme = args.scheme;

        // Create a global flag to indicate if a solution is found
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
                        if args.is_matched(&addr.to_string()) {
                            *catched.write().unwrap() = true;
                            return Some((addr, s));
                        }

                        // Check if we found already found a solution (catched by another thread)
                        if catched.read().unwrap().clone() {
                            return None;
                        }
                    }
                })
            })
            .collect();

        // Wait for the threads to finish
        // Any thread that finds a solution will return it
        // The rest of the threads will return None
        for h in handles {
            if let Ok(Some(solution)) = h.join() {
                return Ok(solution);
            }
        }

        spinner.stop();

        Err(anyhow::anyhow!("No solution found"))
    }
}
