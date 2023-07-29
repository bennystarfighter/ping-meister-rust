use crate::config;

#[cfg(windows)]
pub mod ip_windows;
#[cfg(windows)]
pub use ip_windows as ip;

#[cfg(unix)]
pub mod unix_ip;
#[cfg(unix)]
pub use unix_ip as ip;

use self::ip::query_ip;

mod http;
use self::http::query_http;

#[derive(Clone, Copy)]
pub struct Response {
    pub status: Status,
    pub latency: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Success,
    Fail,
    Unknown,
}

impl config::Target {
    pub fn run_query(&self) -> Response {
        match self.r#type.to_lowercase().as_ref() {
            "ip" => query_ip(self),
            "http" => query_http(self),
            &_ => todo!(),
        }
    }
}
