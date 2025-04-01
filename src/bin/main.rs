use clap::Parser;
use sui_keytool_grinding::args::Grind;

pub fn main() {
    let args = Grind::parse();
    let keytool = sui_keytool_grinding::grind::Keytool::new();
    let (addr, sp) = keytool.grind(args).unwrap();

    println!("====================================================");
    println!("Address:\t{}", addr);
    println!("Seedphase:\t{}", sp);
    println!("====================================================");
}
