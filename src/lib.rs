// `error_chain!` deep recursion.
#![recursion_limit = "1024"]

#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper_tls;
extern crate native_tls;

pub mod cli;
pub mod errors;
pub mod client;
