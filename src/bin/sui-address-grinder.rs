use clap::Parser;
use sui_address_grinder::{args::GrindArgs, grinder::Grinder};

#[tokio::main]
pub async fn main() {
    let args = GrindArgs::parse();
    args.must_validate();

    let grinder = Grinder::new();
    grinder.grind(args);
}
