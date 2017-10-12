#[macro_use]
extern crate error_chain;
extern crate gojira;

use gojira::errors::*;
use gojira::cli::{Action};
use gojira::client;

quick_main!(run);

fn run() -> Result<()> {
    let options = gojira::cli::run();

    match options.action {
        Action::ListProjects => list_projects(),
    }
}

fn list_projects() -> Result<()> {
    client::projects()
}
