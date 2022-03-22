
#![allow(warnings)]
use anchor_lang::prelude::*;
pub mod entrypoint;
pub mod processor;

/// Prefix used in PDA derivations to avoid collisions with other programs.
pub const PREFIX: &str = "ProtectedNFT";
declare_id!("6qZt2RG8og8DKkjXFi3aQCWMg9pkiUUBb4s6nbGgMPry");
