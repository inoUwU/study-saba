extern crate alloc;
use crate::alloc::string::ToString;
use alloc::format;
use alloc::string::String;
use noli::net::lookup_host;
// use saba_core::error::Error;  Todo: Create a custom error type
// use saba_core::http::HttpResponse;  // Todo: Create a custom HttpResponse type

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                    e
                )))
            }
        };

        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
