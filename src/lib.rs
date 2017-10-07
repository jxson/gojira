// `error_chain!` deep recursion.
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

pub mod cli;
pub mod errors;
