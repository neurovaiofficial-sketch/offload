//! One-time pairing codes.
//!
//! A code is eight Crockford base32 symbols shown as `SLOT-SECRET` (e.g. `K7QM-4ZP9`).
//! The slot is allocated by the pairing directory and only tells the other device which
//! computer to reach; the secret never leaves the two devices (it travels to the computer
//! showing the code inside the end-to-end encrypted session) and is what that computer checks.

use crate::crypto;

/// Crockford base32: no I, L, O or U, so the code survives being read aloud or misread.
pub const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
pub const SLOT_LEN: usize = 4;
pub const SECRET_LEN: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeError {
    WrongLength,
    BadSymbol(char),
}

impl std::fmt::Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeError::WrongLength => write!(
                f,
                "a code has {} letters and digits",
                SLOT_LEN + SECRET_LEN
            ),
            CodeError::BadSymbol(c) => write!(f, "'{c}' cannot appear in a code"),
        }
    }
}

impl std::error::Error for CodeError {}

/// Maps a typed character to its canonical symbol, forgiving the usual confusions.
fn canonical(c: char) -> Option<u8> {
    let c = c.to_ascii_uppercase();
    let c = match c {
        'O' => '0',
        'I' | 'L' => '1',
        _ => c,
    };
    if c.is_ascii() && ALPHABET.contains(&(c as u8)) {
        Some(c as u8)
    } else {
        None
    }
}

/// `n` uniformly random symbols from the OS CSPRNG.
pub fn random_symbols(n: usize) -> String {
    (0..n)
        .map(|_| ALPHABET[crypto::random_below(ALPHABET.len() as u32) as usize] as char)
        .collect()
}

pub fn is_valid_slot(slot: &str) -> bool {
    slot.len() == SLOT_LEN && slot.bytes().all(|b| ALPHABET.contains(&b))
}

#[derive(Clone, PartialEq, Eq)]
pub struct PairingCode {
    slot: String,
    secret: String,
}

// The secret must not end up in logs.
impl std::fmt::Debug for PairingCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PairingCode({}-****)", self.slot)
    }
}

impl PairingCode {
    /// A fresh code for a slot the directory handed out.
    pub fn generate(slot: &str) -> Result<Self, CodeError> {
        Self::from_parts(slot, &random_symbols(SECRET_LEN))
    }

    pub fn from_parts(slot: &str, secret: &str) -> Result<Self, CodeError> {
        Self::parse(&format!("{slot}{secret}"))
    }

    /// Parses what a person typed: case, spaces, dashes and look-alike letters are forgiven.
    pub fn parse(input: &str) -> Result<Self, CodeError> {
        let mut symbols = String::with_capacity(SLOT_LEN + SECRET_LEN);
        for c in input.chars() {
            if c.is_whitespace() || c == '-' || c == '_' || c == '.' {
                continue;
            }
            match canonical(c) {
                Some(s) => symbols.push(s as char),
                None => return Err(CodeError::BadSymbol(c)),
            }
            if symbols.len() > SLOT_LEN + SECRET_LEN {
                return Err(CodeError::WrongLength);
            }
        }
        if symbols.len() != SLOT_LEN + SECRET_LEN {
            return Err(CodeError::WrongLength);
        }
        let secret = symbols.split_off(SLOT_LEN);
        Ok(Self {
            slot: symbols,
            secret,
        })
    }

    pub fn slot(&self) -> &str {
        &self.slot
    }

    /// The canonical form without separators, as signed in pairing requests.
    pub fn canonical(&self) -> String {
        format!("{}{}", self.slot, self.secret)
    }

    /// The form shown on screen.
    pub fn display(&self) -> String {
        format!("{}-{}", self.slot, self.secret)
    }

    /// Constant-time comparison of a presented code against this one.
    pub fn matches(&self, other: &PairingCode) -> bool {
        // Both halves are fixed-length, so comparing the concatenation is constant-time.
        crypto::secure_eq(self.canonical().as_bytes(), other.canonical().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_codes_are_forgiving_about_case_spacing_and_look_alikes() {
        let c = PairingCode::parse(" k7qm - 4zp9 ").unwrap();
        assert_eq!(c.display(), "K7QM-4ZP9");
        let o = PairingCode::parse("O1IL-0000").unwrap();
        assert_eq!(o.display(), "0111-0000");
    }

    #[test]
    fn malformed_codes_are_rejected() {
        assert_eq!(PairingCode::parse("K7QM-4ZP"), Err(CodeError::WrongLength));
        assert_eq!(PairingCode::parse("K7QM-4ZP99"), Err(CodeError::WrongLength));
        assert_eq!(PairingCode::parse("K7QM-4ZPU"), Err(CodeError::BadSymbol('U')));
        assert_eq!(PairingCode::parse("K7QM-4ZP\u{e9}"), Err(CodeError::BadSymbol('\u{e9}')));
        assert_eq!(PairingCode::parse(""), Err(CodeError::WrongLength));
    }

    #[test]
    fn generated_codes_are_valid_and_differ() {
        let a = PairingCode::generate("K7QM").unwrap();
        assert_eq!(a.slot(), "K7QM");
        assert_eq!(PairingCode::parse(&a.display()).unwrap(), a);
        let distinct: std::collections::HashSet<String> = (0..50)
            .map(|_| PairingCode::generate("K7QM").unwrap().canonical())
            .collect();
        assert!(distinct.len() > 40);
        assert!(PairingCode::generate("BAD!").is_err());
    }

    #[test]
    fn matching_needs_both_halves() {
        let a = PairingCode::parse("K7QM-4ZP9").unwrap();
        assert!(a.matches(&PairingCode::parse("k7qm4zp9").unwrap()));
        assert!(!a.matches(&PairingCode::parse("K7QM-4ZP8").unwrap()));
        assert!(!a.matches(&PairingCode::parse("K7QN-4ZP9").unwrap()));
    }

    #[test]
    fn debug_output_hides_the_secret() {
        let a = PairingCode::parse("K7QM-4ZP9").unwrap();
        assert!(!format!("{a:?}").contains("4ZP9"));
    }

    #[test]
    fn slot_validation() {
        assert!(is_valid_slot("K7QM"));
        assert!(!is_valid_slot("k7qm"));
        assert!(!is_valid_slot("K7Q"));
        assert!(!is_valid_slot("K7QU"));
    }
}
