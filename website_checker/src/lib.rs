mod config;
mod types;
mod thread;

pub use config::Config;
pub use types::WebsiteStatus;

pub use run_website_checker::run_website_checker;

mod run_website_checker {
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;
    use crate::{Config, WebsiteStatus};
    use crate::thread::thread_loop;
    // runs the website checker sets up the channels and spawns threads distrubtes urls among them
    // returns the final lists of statuses 
    pub fn run_website_checker(urls: Vec<&str>, config: Config) -> Vec<WebsiteStatus> {
        let (work_tx, work_rx) = mpsc::channel::<String>();
        let (result_tx, result_rx) = mpsc::channel::<WebsiteStatus>();

        let work_rx = Arc::new(Mutex::new(work_rx));

        let mut threads = Vec::new();
        //  loop spawns the specified number of threads to process the urls 
        for _ in 0..config.num_threads {
            let thread_rx = Arc::clone(&work_rx);
            let thread_tx = result_tx.clone();
            let cfg = Config {
                num_threads: config.num_threads,
                request_timeout: config.request_timeout,
                max_retries: config.max_retries,
            };

            threads.push(thread::spawn(move || {
                thread_loop(thread_rx, thread_tx, cfg);
            }));
        }

        for url in urls {
            work_tx.send(url.to_string()).unwrap();
        }

        drop(work_tx); 
        drop(result_tx);

        let mut results = Vec::new();
        for result in result_rx {
            results.push(result);
        }

        for w in threads {
            w.join().unwrap();
        }

        results
    }
}
