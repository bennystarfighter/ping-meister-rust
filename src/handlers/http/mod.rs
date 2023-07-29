use crate::config::Target;

use super::{Response, Status};
use reqwest::{self};
use std::time::{Duration, Instant};

pub fn query_http(target: &Target) -> Response {
    let mut return_response: Response = Response {
        status: Status::Fail,
        latency: -1,
    };

    let start = Instant::now();

    let builder = reqwest::blocking::ClientBuilder::new()
        .timeout(Duration::from_secs(u64::from(target.timeout)))
        .user_agent("ping-meister/1.0");

    match builder.build() {
        Ok(client) => {
            match client.get(&target.address).send() {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return_response.status = Status::Success;
                    }

                    let duration = start.elapsed();
                    return_response.latency = duration.as_millis() as i32;
                }
                Err(_) => {
                    return_response.status = Status::Fail;
                }
            };
        }
        Err(error) => panic!("{:}", error),
    }
    return_response
}
