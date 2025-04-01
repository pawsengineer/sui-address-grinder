use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Grind {
    /// Specific a pubkey to start with
    #[arg(short, long)]
    starts_with: Option<String>,

    /// Specific a pubkey to ends with
    #[arg(short, long)]
    ends_with: Option<String>,

    /// Ignore case-sensitivity
    #[arg(short, long)]
    ignore_case: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with() {
        let args = Grind {
            starts_with: Some("123".to_string()),
            ends_with: None,
            ignore_case: false,
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
        };
        assert!(args.is_valid("0x123ABC"));
        assert!(args.is_valid("0x123abc"));
    }
}
