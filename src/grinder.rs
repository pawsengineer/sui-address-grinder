use crate::helper::u64::Floorer;
use crate::{args::GrindArgs, helper::cores::get_core_ids};
use std::{
    process::exit,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use spinoff::{Color, Spinner, spinners};
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
        let core_size = core_ids.len();
        let scheme = args.scheme;

        let mut spinner = Spinner::new(
            spinners::Dots6,
            format!("Grinding using {} cores...", core_size),
            Color::Blue,
        );

        let (sender, mut recv) = tokio::sync::mpsc::unbounded_channel::<Solution>();
        tokio::spawn({
            async move {
                while let Some(solution) = recv.recv().await {
                    println!();
                    println!("====================================================");
                    println!("Address:\t{}", solution.0);
                    println!("Seed phrase:\t{}", solution.1);
                    println!("====================================================");
                    exit(0);
                }
            }
        });

        let total_grinded = Arc::new(AtomicU64::new(0));

        let handles: Vec<_> = core_ids
            .into_iter()
            .map(|i| {
                let args = args.clone();
                let sender = sender.clone();
                let total_grinded = total_grinded.clone();

                std::thread::spawn(move || {
                    // Pin to core
                    let _ = core_affinity::set_for_current(i);

                    loop {
                        // Check if the read mux is closed
                        if sender.is_closed() {
                            println!("Sender is closed, exiting...");
                            exit(0);
                        }

                        // Otherwise, generate a new key
                        let ret = InMemKeystore::default()
                            .generate_and_add_new_key(scheme, None, None, None);
                        if ret.is_err() {
                            println!("Error generating key: {}", ret.unwrap_err());
                            continue;
                        }

                        let (addr, sp, _) = ret.unwrap();

                        if args.is_matched(&addr.to_string()) {
                            let _ = sender.send(Solution(addr, sp));
                        }

                        total_grinded.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();

        // Periodically update the overall grind count in the spinner
        let total_grinded_clone = total_grinded.clone();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(5));
                spinner.update(
                    spinners::Dots2,
                    format!(
                        "Grinding using {} cores... (processed {} keys)",
                        core_size,
                        total_grinded_clone
                            .load(Ordering::Relaxed)
                            .reduce_to_significant_digit()
                    ),
                    None,
                );
            }
        });

        // Wait for the threads to finish
        // Any thread that finds a solution will return it
        // The rest of the threads will return None
        for h in handles {
            h.join().expect("Thread panicked");
        }
    }
}
