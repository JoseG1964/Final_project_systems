use website_checker::{run_website_checker, Config};
use std::time::Duration;
use std::fs;

fn main() {

    let contents = fs::read_to_string("websites.txt")
        .expect("Failed to read websites.txt");
    let websites: Vec<&str> = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty()) // ignore empty lines
        .collect();


    let config = Config {
        num_threads: 50, //threads
        request_timeout: Duration::from_secs(5), //timeout for sites
        max_retries: 2,
    };

    let results = run_website_checker(websites, config);

    let mut num = 0;
    for res in results {
        num += 1;
        println!("{} {:?}", num, res);
    }
}
