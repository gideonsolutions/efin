use std::fmt;
use std::str::FromStr;

/// A 6-digit IRS Electronic Filing Identification Number (EFIN).
///
/// An EFIN is assigned by the IRS to authorized e-file providers. It is always
/// exactly 6 ASCII digits (e.g. `"007777"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Efin(String);

/// Error returned when an EFIN string is invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEfinError {
    _private: (),
}

impl fmt::Display for ParseEfinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid EFIN: must be exactly 6 ASCII digits")
    }
}

impl std::error::Error for ParseEfinError {}

impl Efin {
    /// Create a new `Efin` after validating the input.
    pub fn new(s: &str) -> Result<Self, ParseEfinError> {
        s.parse()
    }

    /// Return the EFIN as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Efin {
    type Err = ParseEfinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 6 && s.bytes().all(|b| b.is_ascii_digit()) {
            Ok(Self(s.to_owned()))
        } else {
            Err(ParseEfinError { _private: () })
        }
    }
}

impl fmt::Display for Efin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Efin {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_efin() {
        let e = Efin::new("007777").unwrap();
        assert_eq!(e.as_str(), "007777");
        assert_eq!(e.to_string(), "007777");
    }

    #[test]
    fn too_short() {
        assert!(Efin::new("12345").is_err());
    }

    #[test]
    fn too_long() {
        assert!(Efin::new("1234567").is_err());
    }

    #[test]
    fn non_digit() {
        assert!(Efin::new("abcdef").is_err());
    }

    #[test]
    fn from_str_trait() {
        let e: Efin = "001234".parse().unwrap();
        assert_eq!(e.as_str(), "001234");
    }

    #[test]
    fn equality_and_ord() {
        let a = Efin::new("000001").unwrap();
        let b = Efin::new("000002").unwrap();
        assert!(a < b);
        assert_eq!(a, a.clone());
    }
}
