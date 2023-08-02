use crate::config::Target;

use super::Response;
use std::time::{Duration, Instant};

pub fn query_http(target: &Target) -> Response {
    let start = Instant::now();

    let builder = reqwest::blocking::ClientBuilder::new()
        .timeout(Duration::from_secs(u64::from(target.timeout)))
        .user_agent("ping-meister/1.0");

    let client = builder.build().unwrap();

    match client.get(&target.address).send() {
        Ok(_) => Response::Success {
            latency: start.elapsed().as_millis() as i32,
        },
        Err(_) => Response::Failure,
    }
}
