use std::time::Duration;

pub struct Config {
    pub num_threads: usize,
    pub request_timeout: Duration,
    pub max_retries: usize,
}
