use clap::Parser;
use sui_keytool_grinding::{args::GrindArgs, grinder::Grinder};

pub fn main() {
    let args = GrindArgs::parse();
    args.must_validate();

    let grinder = Grinder::new();
    let (addr, sp) = grinder.grind(args).unwrap();

    println!();
    println!("====================================================");
    println!("Address:\t{}", addr);
    println!("Seedphase:\t{}", sp);
    println!("====================================================");
}
