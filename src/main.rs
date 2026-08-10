use std::env;
use reqwest::blocking::Client;
use std::time::Duration;

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
    let result = client.get(&arg).send();
    match result {
        Ok(response) => {
            if response.status().is_success() {
                println!("Request successful for {arg} with a status code of {}", response.status());
            } else {
                eprintln!("Request failed for {arg} with status: {}", response.status());
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error making request for {arg} with error of: {}", e);
            std::process::exit(1);
        }
    }
}
