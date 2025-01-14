//! `Rstr`:
//! - is meant to be a bundler-specialized string type for rolldown.
//! - to smooth integration with `oxc`'s string types.

use std::{fmt::Display, ops::Deref};

/// `OxcStr` is a alias of string type oxc used internally.
pub type OxcStr = oxc::span::CompactString;

mod to_str;
pub use to_str::ToRstr;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct Rstr(OxcStr);

impl Rstr {
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }
}

impl Deref for Rstr {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.as_str()
  }
}

impl Display for Rstr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.as_str().fmt(f)
  }
}
