use clap::Parser;
use sui_keytool_grinding::{args::Grind, grind::Keytool};

pub fn main() {
    let args = Grind::parse();
    args.must_validate();

    let keytool = Keytool::new();
    let (addr, sp) = keytool.grind(args).unwrap();

    println!("====================================================");
    println!("Address:\t{}", addr);
    println!("Seedphase:\t{}", sp);
    println!("====================================================");
}
