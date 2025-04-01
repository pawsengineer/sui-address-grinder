use clap::Parser;
use sui_types::crypto::SignatureScheme;

use crate::helper::signature_scheme::SignatureSchemeArg;

#[derive(Parser)]
#[command(name = "Sui Keytool Grinder")]
#[command(version = "1.0")]
#[command(about, long_about = None)]
pub struct Grind {
    /// Specific a pubkey to start with
    #[arg(long)]
    starts_with: Option<String>,

    /// Specific a pubkey to ends with
    #[arg(long)]
    ends_with: Option<String>,

    /// Ignore case-sensitivity
    #[arg(long)]
    ignore_case: bool,

    /// Specific a signature scheme (secp256k1, secp256r1, ed25519)
    #[arg(long, default_value = "ed25519", value_parser = Grind::try_from_arg)]
    pub scheme: SignatureScheme,
}

impl Default for Grind {
    fn default() -> Self {
        Grind {
            starts_with: None,
            ends_with: None,
            ignore_case: false,
            scheme: SignatureScheme::ED25519,
        }
    }
}

impl Grind {
    pub fn is_valid(&self, addr: &str) -> bool {
        let mut addr = addr.to_string();
        if addr.starts_with("0x") {
            addr = addr[2..].to_string();
        }

        if self.ignore_case {
            addr = addr.to_lowercase();
        }

        if let Some(starts_with) = &self.starts_with {
            let mut starts_with = starts_with.clone();
            if self.ignore_case {
                starts_with = starts_with.to_lowercase();
            }
            if !addr.starts_with(&starts_with) {
                return false;
            }
        }

        if let Some(ends_with) = &self.ends_with {
            let mut ends_with = ends_with.clone();
            if self.ignore_case {
                ends_with = ends_with.to_lowercase();
            }
            if !addr.ends_with(&ends_with) {
                return false;
            }
        }

        true
    }
}

impl SignatureSchemeArg for Grind {
    fn try_from_arg(s: &str) -> Result<SignatureScheme, anyhow::Error> {
        match s {
            "ed25519" => Ok(SignatureScheme::ED25519),
            "secp256k1" => Ok(SignatureScheme::Secp256k1),
            "secp256r1" => Ok(SignatureScheme::Secp256r1),
            _ => Err(anyhow::anyhow!("Unknown signature scheme: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with() {
        let args = Grind {
            starts_with: Some("123".to_string()),
            ends_with: None,
            ignore_case: false,
            ..Default::default()
        };
        assert!(!args.is_valid("0x0123abc"));
        assert!(args.is_valid("0x123abc"));
    }

    #[test]
    fn test_ends_with() {
        let args = Grind {
            starts_with: None,
            ends_with: Some("abc".to_string()),
            ignore_case: false,
            ..Default::default()
        };
        assert!(!args.is_valid("0x123abc0"));
        assert!(args.is_valid("0x123abc"));
    }

    #[test]
    fn test_starts_ends_with() {
        let args = Grind {
            starts_with: Some("123".to_string()),
            ends_with: Some("abc".to_string()),
            ignore_case: false,
            ..Default::default()
        };
        assert!(!args.is_valid("0xabc123"));
        assert!(args.is_valid("0x123abc"));
    }

    #[test]
    fn test_ignore_case() {
        let args = Grind {
            starts_with: Some("123".to_string()),
            ends_with: Some("abc".to_string()),
            ignore_case: true,
            ..Default::default()
        };
        assert!(args.is_valid("0x123ABC"));
        assert!(args.is_valid("0x123abc"));
    }

    #[test]
    fn test_scheme_from_arg() {
        assert_eq!(
            Grind::try_from_arg("ed25519").unwrap(),
            sui_types::crypto::SignatureScheme::ED25519
        );
        assert_eq!(
            Grind::try_from_arg("secp256k1").unwrap(),
            sui_types::crypto::SignatureScheme::Secp256k1
        );
        assert_eq!(
            Grind::try_from_arg("secp256r1").unwrap(),
            sui_types::crypto::SignatureScheme::Secp256r1
        );
        assert!(Grind::try_from_arg("unknown").is_err());
    }
}
