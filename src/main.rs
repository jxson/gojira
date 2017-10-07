#[macro_use]
extern crate error_chain;
extern crate gojira;

use gojira::errors::*;
use gojira::cli::{Action};

quick_main!(run);

fn run() -> Result<()> {
    let options = gojira::cli::run();

    match options.action {
        Action::ListProjects => list_projects(),
    }

    Ok(())
}

fn list_projects() {
    println!("Should list projects.");
}
