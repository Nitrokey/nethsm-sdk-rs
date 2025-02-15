// ureq v2 has a large error type, thus our errors are large too.
// This will be fixed by updating to ureq v3.
#![allow(clippy::result_large_err)]

#[macro_use]
extern crate serde_derive;

pub use ureq;

pub mod apis;
pub mod models;
