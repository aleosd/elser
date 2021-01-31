use clap::{App, Arg};

pub fn parse_args() -> clap::ArgMatches {
    return App::new("ElasticSearch utility")
        .version("0.1")
        .author("Alex Osadchuk <osdalex@gmail.com>")
        .about("Implements some basic request to maintain elasticseasrch state")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::new("log-level")
                .short('l')
                .long("log-level")
                .takes_value(true)
                .possible_values(&["info", "debug", "warn", "error"])
                .about("Set log level"),
        )
        .get_matches();
}
