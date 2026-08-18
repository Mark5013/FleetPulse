use reqwest::blocking::Client;
use std::env;
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug)]
enum FailureReason {
    InvalidUrl,
    Timeout,
    Connection,
    Other,
}

#[derive(Debug)]
enum CheckOutcome {
    Healthy { status_code: reqwest::StatusCode },
    UnhealthyHttp { status_code: reqwest::StatusCode },
    RequestFailed { reason: FailureReason },
}

#[derive(Debug)]
struct CheckResult {
    url: String,
    checked_at: SystemTime,
    latency: Duration,
    outcome: CheckOutcome,
}

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
    let result = check_url(&client, &arg);
    let elapsed_time_ms = started.elapsed().as_millis();
    match result {
        Ok(status) => {
            if status.is_success() {
                println!(
                    "Request successful for {arg} with a status code of {} (took {} ms)",
                    status, elapsed_time_ms
                );
            } else {
                eprintln!(
                    "Request failed for {arg} with status: {} (took {} ms)",
                    status, elapsed_time_ms
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

fn check_url(client: &Client, url: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
    let response = client.get(url).send()?;
    Ok(response.status())
}

fn classify_status(status_code: reqwest::StatusCode) -> CheckOutcome {
    if status_code.is_success() {
        CheckOutcome::Healthy { status_code }
    } else {
        CheckOutcome::UnhealthyHttp { status_code }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_status_healthy_status() {
        let status_code = reqwest::StatusCode::OK;

        let outcome = classify_status(status_code);

        assert!(matches!(
            outcome,
            CheckOutcome::Healthy {
                status_code: reqwest::StatusCode::OK
            }
        ));
    }

    #[test]
    fn test_classify_status_unhealthy_status() {
        let status_code = reqwest::StatusCode::INTERNAL_SERVER_ERROR;

        let outcome = classify_status(status_code);

        assert!(matches!(
            outcome,
            CheckOutcome::UnhealthyHttp {
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR
            }
        ));
    }
}
