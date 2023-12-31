//! Named symbols for use in compute algebra systems. Symbol names are
//! stored centrally such that copies are cheap and need little
//! memory.
//!
//! # Example
//!
//! ```rust
//! use math_symbols::*;
//!
//! // Define a number of symbols with variable name equal to symbol name
//! symbols!(x, y, z);
//! assert_eq!(x.name(), "x");
//! assert_eq!(y.name(), "y");
//! assert_eq!(z.name(), "z");
//!
//! // Symbols are identified by their names
//! let xx = Symbol::new("x");
//! assert_eq!(x, xx);
//!
//! // Symbols are ordered by their creation time
//! assert!(x < y);
//! ```
//! # Similar crates
//!
//! - [symbol](https://crates.io/crates/symbol)
//!
use std::fmt::{self, Display};
use std::sync::RwLock;

use ahash::AHashMap;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct SymbolRegister {
    names: Vec<String>,
    indices: AHashMap<String, usize>,
}

impl SymbolRegister {
    fn name(&self, idx: usize) -> &str {
        &self.names[idx]
    }

    fn try_idx(&self, name: &str) -> Option<usize> {
        self.indices.get(name).copied()
    }

    fn idx(&mut self, name: &str) -> usize {
        if let Some(idx) = self.try_idx(name) {
            return idx;
        }
        let new_idx = self.names.len();
        self.indices.insert(String::from(name), new_idx);
        self.names.push(String::from(name));
        new_idx
    }
}

lazy_static! {
    static ref SYMBOL_REGISTER: RwLock<SymbolRegister> =
        RwLock::new(SymbolRegister::default());
}

/// A symbol
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Deserialize,
    Serialize,
)]
#[serde(transparent)]
pub struct Symbol {
    #[serde(
        serialize_with = "serialize_sym",
        deserialize_with = "deserialize_sym"
    )]
    idx: usize,
}

impl Symbol {
    /// Construct a symbol with the given name
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = name.as_ref();
        if let Some(idx) = SYMBOL_REGISTER.read().unwrap().try_idx(name) {
            return Self { idx };
        }
        let idx = SYMBOL_REGISTER.write().unwrap().idx(name);
        Self { idx }
    }

    /// Get the symbol's name
    pub fn name(&self) -> String {
        SYMBOL_REGISTER.read().unwrap().name(self.idx).to_owned()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", SYMBOL_REGISTER.read().unwrap().name(self.idx))
    }
}

fn serialize_sym<S: Serializer>(sym: &usize, s: S) -> Result<S::Ok, S::Error> {
    let sym = Symbol { idx: *sym };
    let name = sym.name();
    String::serialize(&name, s)
}

fn deserialize_sym<'de, D: Deserializer<'de>>(d: D) -> Result<usize, D::Error> {
    let name = String::deserialize(d)?;
    let s = Symbol::new(&name);
    Ok(s.idx)
}

/// Construct variables with the same variable and symbol name
#[macro_export]
macro_rules! symbols {
    ( $( $x:ident ),* ) => {
        $(
            let $x = Symbol::new(stringify!($x));
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol() {
        symbols!(x, y, z);
        assert_eq!(x.name(), "x");
        assert_eq!(y.name(), "y");
        assert_eq!(z.name(), "z");
        let xx = Symbol::new("x");
        assert_eq!(xx.name(), "x");
    }
}
