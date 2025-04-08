use std::process::exit;

use crate::{args::GrindArgs, helper::cores::get_core_ids};

use spinners::{Spinner, Spinners};
use sui_keys::keystore::{AccountKeystore, InMemKeystore};
use sui_types::base_types::SuiAddress;

pub struct Solution(pub SuiAddress, pub String);

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
    pub fn grind(&self, args: GrindArgs) {
        let core_ids = get_core_ids(args.cores);
        let scheme = args.scheme;

        let mut spinner = Spinner::new(
            Spinners::Dots8,
            format!("Grinding using {} cores...", core_ids.len()),
        );

        let (sender, mut recv) = tokio::sync::mpsc::unbounded_channel::<Solution>();
        tokio::spawn({
            async move {
                while let Some(solution) = recv.recv().await {
                    spinner.stop();
                    println!();
                    println!("====================================================");
                    println!("Address:\t{}", solution.0);
                    println!("Seedphrase:\t{}", solution.1);
                    println!("====================================================");
                    exit(0);
                }
            }
        });

        let handles: Vec<_> = core_ids
            .into_iter()
            .map(|i| {
                let args = args.clone();
                let sender = sender.clone();

                std::thread::spawn(move || {
                    // Pin to core
                    let _ = core_affinity::set_for_current(i);

                    let mut key_store = InMemKeystore::default();
                    loop {
                        // Check if the read mux is closed
                        if sender.is_closed() {
                            exit(0);
                        }

                        // Otherwise, generate a new key
                        let ret = key_store.generate_and_add_new_key(scheme, None, None, None);
                        if ret.is_err() {
                            continue;
                        }

                        let (addr, sp, _) = ret.unwrap();
                        if args.is_matched(&addr.to_string()) {
                            let _ = sender.send(Solution(addr, sp));
                        }
                    }
                })
            })
            .collect();

        // Wait for the threads to finish
        // Any thread that finds a solution will return it
        // The rest of the threads will return None
        for h in handles {
            if let Ok(()) = h.join() {}
        }
    }
}
