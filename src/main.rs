use reqwest::blocking::Client;
use std::env;
use std::time::{Duration, Instant};

fn main() {
    if env::args().len() != 2 {
        eprintln!("Incorrect number of arguments. Please provide only one argument.");
        std::process::exit(1);
    }
    let arg = env::args().nth(1).unwrap();
    let client_result = Client::builder().timeout(Duration::from_secs(5)).build();
    let client = match client_result {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error creating HTTP client: {}", e);
            std::process::exit(1);
        }
    };
    let started = Instant::now();
    let result = client.get(&arg).send();
    let elapsed_time_ms = started.elapsed().as_millis();
    match result {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Request successful for {arg} with a status code of {} (took {} ms)",
                    response.status(),
                    elapsed_time_ms
                );
            } else {
                eprintln!(
                    "Request failed for {arg} with status: {} (took {} ms)",
                    response.status(),
                    elapsed_time_ms
                );
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!(
                "Error making request for {arg} with error of: {} (took {} ms)",
                e, elapsed_time_ms
            );
            std::process::exit(1);
        }
    }
}
