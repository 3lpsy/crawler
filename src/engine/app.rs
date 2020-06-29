use super::config::AppConfig;
use super::pool::ThreadPool;
use std::thread::sleep;
use std::time::Duration;

pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { config: config }
    }

    pub fn run(&self) {
        let pool = ThreadPool::new(self.config.thread_pool_limit as usize);
        let urls = self.config.urls.clone();
        for url in urls {
            pool.execute(move || {
                println!("Starting callback: {:?}", url);
                sleep(Duration::from_secs(4));
                println!("Callback done: {:?}", url);
            });
        }
        pool.join();
    }
}
