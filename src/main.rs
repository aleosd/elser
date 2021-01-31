use log::debug;
use std::path::PathBuf;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod cli;
mod logger;
mod settings;

fn main() {
    let args = cli::parse_args();
    logger::setup_logger(&args.value_of("log-level").unwrap_or("warn"));
    let settings = settings::Settings::load(args.value_of_t::<PathBuf>("config").ok());
    debug!("Successfully loaded config");
    println!("{:?}", settings)
}
