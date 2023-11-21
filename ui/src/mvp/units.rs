//! Utilities for types, units, and other things that we have to be careful
//! about.

use super::*;

/// Addresses can easily get "displayed" or "debugged" into a string with
/// ellipses in the middle. That's bad.
pub fn address_to_string(address: &Address) -> String {
    format!("0x{:x}", address)
}
