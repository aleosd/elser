use log::debug;
use std::path::PathBuf;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod cli;
mod elastic;
mod logger;
mod settings;

#[tokio::main]
async fn main() {
    let args = cli::parse_args();
    logger::setup_logger(&args.value_of("log-level").unwrap_or("warn"));
    let settings = settings::Settings::load(args.value_of_t::<PathBuf>("config").ok()).unwrap();
    debug!("Successfully loaded config");
    let elastic_connection = &settings.get_connection(Some("default"));
    println!("{:?}", elastic_connection);
    let elastic_client =
        elastic::client::get_elastic_client(elastic_connection.as_ref().unwrap()).unwrap();
    let cluster_info = elastic_client.info();
    let response = cluster_info.send().await.unwrap();
    let content = response.text().await.unwrap();
    println!("{}", content);
}
