use std::sync::{Arc, Mutex, mpsc::{Receiver, Sender}};
use std::time::{Duration, Instant};
use crate::{WebsiteStatus, Config};
use chrono::{DateTime, Utc};

pub fn thread_loop(work_rx: Arc<Mutex<Receiver<String>>>, result_tx: Sender<WebsiteStatus>, config: Config) {
    loop {
        let url = {
            let lock = work_rx.lock().unwrap();
            match lock.recv() {
                Ok(u) => u,
                Err(_) => break, 
            }
        };

        let start_time = Instant::now();
        let result = check_website(&url, config.request_timeout, config.max_retries);
        let duration = start_time.elapsed();

        let status = WebsiteStatus {
            url: url,
            status: result.map_err(|e| e.to_string()),
            response_time: duration,
            timestamp: Utc::now(),
        };

        if result_tx.send(status).is_err() {
            break;
        }
    }
}

//tries to get website http status code within timeout time and number of tries. returns error if fails
fn check_website(url: &str, timeout: Duration, max_retries: usize) -> Result<u16, String> {
    let agent = ureq::builder()
        .timeout_connect(timeout)
        .timeout_read(timeout)
        .build();

    let mut attempts = 0;
    loop {
        attempts += 1;
        let resp = agent.get(url).call();
        match resp {
            Ok(r) => return Ok(r.status()),
            Err(e) => {
                if attempts >= max_retries {
                    return Err(format!("Failed after {} attempts: {}", attempts, e));
                }
            }
        }
    }
}
