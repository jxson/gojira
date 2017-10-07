use clap;

mod app;

pub fn run<'a>() -> Options {
    let matches = self::app::new().get_matches();
    Options::new(matches)
}

fn run_with<'a>(args: Vec<&str>) -> Options {
    let matches = self::app::new().get_matches_from(args);
    Options::new(matches)
}


#[derive(Debug, PartialEq)]
pub enum Action {
    ListProjects,
}

pub struct Options {
    pub action: Action,
}

impl Options {
    pub fn new(matches: clap::ArgMatches) -> Options {
        Options {
            action: Action::ListProjects,
        }
    }
}
