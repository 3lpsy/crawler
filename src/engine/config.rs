use url::Url;

pub const DEFAULT_ASYNC_POOL_LIMIT: u32 = 10;
pub const DEFAULT_THREAD_POOL_LIMIT: u32 = 2;

#[derive(Debug)]
pub struct AppConfig {
    pub urls: Vec<Url>,
    pub async_pool_limit: u32,
    pub thread_pool_limit: u32,
}

impl AppConfig {
    pub fn new(urls: Vec<Url>) -> AppConfig {
        AppConfig {
            urls: urls,
            async_pool_limit: DEFAULT_ASYNC_POOL_LIMIT,
            thread_pool_limit: DEFAULT_THREAD_POOL_LIMIT,
        }
    }

    pub fn set_async_limit(&mut self, limit: u32) {
        self.async_pool_limit = limit;
    }

    pub fn set_thread_limit(&mut self, limit: u32) {
        self.thread_pool_limit = limit;
    }
}
