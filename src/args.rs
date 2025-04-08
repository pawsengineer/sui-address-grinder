use anyhow::Result;
use clap::Parser;
use sui_types::crypto::SignatureScheme;

use crate::{error::GrindArgError, helper::signature_scheme::SignatureSchemeArg};

#[derive(Parser, Clone, Debug)]
#[command(name = "Sui Address Grinder")]
#[command(version = "1.0")]
#[command(about, long_about = None)]
pub struct GrindArgs {
    /// Specific a pubkey to start with
    #[arg(long)]
    pub starts_with: Option<String>,

    /// Specific a pubkey to ends with
    #[arg(long)]
    pub ends_with: Option<String>,

    /// Ignore case-sensitivity
    #[arg(long)]
    pub ignore_case: bool,

    /// Verbose mode
    #[arg(long)]
    pub verbose: bool,

    /// Number of cores to use
    /// If not specified, it will use all available cores
    #[arg(long)]
    pub cores: Option<usize>,

    /// Specific a signature scheme (secp256k1, secp256r1, ed25519)
    #[arg(long, default_value = "ed25519", value_parser = GrindArgs::try_from_arg)]
    pub scheme: SignatureScheme,
}

impl Default for GrindArgs {
    fn default() -> Self {
        GrindArgs {
            starts_with: None,
            ends_with: None,
            ignore_case: false,
            verbose: false,
            cores: None,
            scheme: SignatureScheme::ED25519,
        }
    }
}

impl GrindArgs {
    /// Checks if the string is valid hexadecimal (with or without `0x` prefix)
    fn is_valid_hex(s: &str) -> bool {
        let s = s.strip_prefix("0x").unwrap_or(s); // Remove "0x" if present
        !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Validates all the arguments
    /// Returns an error if any of the arguments are invalid
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(starts_with) = &self.starts_with {
            if !Self::is_valid_hex(starts_with) {
                return Err(GrindArgError::InvalidHexStringStartsWith.into());
            }
        }

        if let Some(ends_with) = &self.ends_with {
            if !Self::is_valid_hex(ends_with) {
                return Err(GrindArgError::InvalidHexStringEndsWith.into());
            }
        }

        Ok(())
    }

    /// Validates all the arguments and exits the program if any of the arguments are invalid
    pub fn must_validate(&self) {
        if let Err(e) = self.validate() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    /// Checks if the address is valid based on the specified criteria
    pub fn is_matched(&self, addr: &String, key: &String) -> bool {
        let mut addr = addr.strip_prefix("0x").unwrap_or(&addr).to_string();

        let mut starts_with = self.starts_with.clone().unwrap_or("".to_string());
        let mut ends_with = self.ends_with.clone().unwrap_or("".to_string());

        if self.ignore_case {
            addr = addr.to_lowercase();
            starts_with = starts_with.to_lowercase();
            ends_with = ends_with.to_lowercase();
        }

        if self.verbose {
            println!("====================================================");
            println!("addr: {}", addr);
            println!("seedphase: {}", key);
            println!("====================================================");
            println!();
        }

        if !addr.starts_with(&starts_with) {
            return false;
        }

        if !addr.ends_with(&ends_with) {
            return false;
        }

        true
    }
}

impl SignatureSchemeArg for GrindArgs {
    fn try_from_arg(s: &str) -> Result<SignatureScheme> {
        match s {
            "ed25519" => Ok(SignatureScheme::ED25519),
            "secp256k1" => Ok(SignatureScheme::Secp256k1),
            "secp256r1" => Ok(SignatureScheme::Secp256r1),
            _ => Err(GrindArgError::InvalidSignatureScheme(s.to_string()).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::GrindArgError;

    use super::*;

    #[test]
    fn test_starts_with() {
        let args = GrindArgs {
            starts_with: Some("123".to_string()),
            ends_with: None,
            ignore_case: false,
            ..Default::default()
        };
        assert!(!args.is_matched(&"0x0123abc".to_string(), &"0x0123abc".to_string()));
        assert!(args.is_matched(&"0x123abc".to_string(), &"0x0123abc".to_string()));
    }

    #[test]
    fn test_ends_with() {
        let args = GrindArgs {
            starts_with: None,
            ends_with: Some("abc".to_string()),
            ignore_case: false,
            ..Default::default()
        };
        assert!(!args.is_matched(&"0x123abc0".to_string(), &"0x0123abc".to_string()));
        assert!(args.is_matched(&"0x123abc".to_string(), &"0x0123abc".to_string()));
    }

    #[test]
    fn test_starts_ends_with() {
        let args = GrindArgs {
            starts_with: Some("123".to_string()),
            ends_with: Some("abc".to_string()),
            ignore_case: false,
            ..Default::default()
        };
        assert!(!args.is_matched(&"0xabc123".to_string(), &"0x0123abc".to_string()));
        assert!(args.is_matched(&"0x123abc".to_string(), &"0x0123abc".to_string()));
    }

    #[test]
    fn test_ignore_case() {
        let args = GrindArgs {
            starts_with: Some("123".to_string()),
            ends_with: Some("abc".to_string()),
            ignore_case: true,
            ..Default::default()
        };
        assert!(args.is_matched(&"0x123ABC".to_string(), &"0x0123abc".to_string()));
        assert!(args.is_matched(&"0x123abc".to_string(), &"0x0123abc".to_string()));
    }

    #[test]
    fn test_scheme_from_arg() {
        assert_eq!(
            GrindArgs::try_from_arg("ed25519").unwrap(),
            sui_types::crypto::SignatureScheme::ED25519
        );
        assert_eq!(
            GrindArgs::try_from_arg("secp256k1").unwrap(),
            sui_types::crypto::SignatureScheme::Secp256k1
        );
        assert_eq!(
            GrindArgs::try_from_arg("secp256r1").unwrap(),
            sui_types::crypto::SignatureScheme::Secp256r1
        );
        assert_eq!(
            GrindArgs::try_from_arg("unknown")
                .unwrap_err()
                .downcast_ref::<GrindArgError>(),
            Some(&GrindArgError::InvalidSignatureScheme(
                "unknown".to_string()
            ))
        );
    }

    #[test]
    fn test_is_valid_hex() {
        assert!(GrindArgs::is_valid_hex("0x123abc"));
        assert!(GrindArgs::is_valid_hex("123abc"));
        assert!(!GrindArgs::is_valid_hex("0x123xyz"));
        assert!(!GrindArgs::is_valid_hex("123xyz"));
        assert!(!GrindArgs::is_valid_hex(""));
    }

    #[test]
    fn test_validate() {
        let args = GrindArgs {
            starts_with: Some("123".to_string()),
            ends_with: Some("abc".to_string()),
            ignore_case: false,
            ..Default::default()
        };
        assert!(args.validate().is_ok());

        let args_invalid = GrindArgs {
            starts_with: Some("xyz".to_string()),
            ends_with: Some("123".to_string()),
            ignore_case: false,
            ..Default::default()
        };
        assert_eq!(
            args_invalid
                .validate()
                .unwrap_err()
                .downcast_ref::<GrindArgError>(),
            Some(&GrindArgError::InvalidHexStringStartsWith)
        );

        let args_invalid = GrindArgs {
            starts_with: Some("123".to_string()),
            ends_with: Some("xyz".to_string()),
            ignore_case: false,
            ..Default::default()
        };
        assert_eq!(
            args_invalid
                .validate()
                .unwrap_err()
                .downcast_ref::<GrindArgError>(),
            Some(&GrindArgError::InvalidHexStringEndsWith)
        );
    }
}
