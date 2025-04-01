use anyhow::Error;
use sui_types::crypto::SignatureScheme;

pub trait SignatureSchemeArg {
    fn try_from_arg(s: &str) -> Result<SignatureScheme, Error>;
}
