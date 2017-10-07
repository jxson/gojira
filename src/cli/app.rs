use clap;

pub fn new<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("gojira")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A basic CLI interface to the Jira cloud.")
        .arg(clap::Arg::with_name("projects")
            .long("projects")
            .short("p")
            .help("List all projects")
            .takes_value(false))
}
