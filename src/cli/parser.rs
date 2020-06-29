use clap::ArgMatches;
use clap::{App, Arg};

pub fn parse() -> ArgMatches {
    let parser = App::new("hhcrawler")
        .version("1.0")
        .author("@3lpsy")
        .about("Does awesome things")
        .arg(
            Arg::with_name("url")
                .short('u')
                .long("url")
                .value_name("URLS")
                .multiple(true)
                .about("Urls to crawl (can be passed multiple times)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("async-limit")
                .short('a')
                .long("async")
                .value_name("ASYNC_LIMIT")
                .about("Limit of concurrent async requests per thread")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("thread-limit")
                .short('t')
                .long("thread")
                .value_name("THREAD_LIMIT")
                .about("Limit of concurrent threads")
                .takes_value(true),
        );
    parser.get_matches()
}
