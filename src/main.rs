mod cli;
mod engine;

use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parser::parse();
    let raw_urls: Vec<&str> = args.values_of("url").unwrap().collect();
    let mut urls: Vec<Url> = Vec::new();

    for raw_url in raw_urls.iter() {
        let parsed_url = Url::parse(raw_url);
        let url = match parsed_url {
            Ok(url) => url,
            Err(error) => panic!("Cannot parse url {}. Received error {:?}", raw_url, error),
        };
        urls.push(url);
    }

    let mut config = engine::config::AppConfig::new(urls);

    let async_limit = args
        .value_of_t("async-limit")
        .unwrap_or(engine::config::DEFAULT_ASYNC_POOL_LIMIT);
    config.set_async_limit(async_limit);

    let thread_limit = args
        .value_of_t("thread-limit")
        .unwrap_or(engine::config::DEFAULT_THREAD_POOL_LIMIT);

    config.set_thread_limit(thread_limit);

    println!("{:?}", config);

    let app = engine::app::App::new(config);
    app.run();
    Ok(())
}
