#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod c_apis;
mod cpu;
mod gb;

pub use c_apis::*;
