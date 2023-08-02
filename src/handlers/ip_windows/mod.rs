use dns_lookup::lookup_host;

use crate::config::Target;
use winping::{Buffer, Pinger};

use super::Response;

pub fn query_ip(target: &Target) -> Response {
    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::new();

    let ip_address: std::net::IpAddr = match lookup_host(&target.address) {
        Ok(ip) => {
            if !ip.is_empty() {
                ip[0]
            } else {
                panic!("Did not get an address from {:}", &target.address)
            }
        }
        Err(error) => panic!("{:}", error),
    };

    let mut successful_pings: i8 = 0;
    let mut latencies: Vec<u32> = Vec::new();

    for _ in 0..4 {
        if let Ok(latency) = pinger.send(ip_address, &mut buffer) {
            successful_pings += 1;
            latencies.push(latency);
        }
    }

    if successful_pings > 2 {
       return Response::Success { latency: average(&latencies) as i32 }
    }

    Response::Failure
}

fn average(numbers: &[u32]) -> u32 {
    let sum: u32 = numbers.iter().sum();
    let count = numbers.len() as u32;
    sum / count
}
