use crate::config::Server;
use serde::Serialize;
use std::fmt;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum PortStatus {
    Open,
    Closed,
    Timeout,
    Error,
}

impl fmt::Display for PortStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortStatus::Open => write!(formatter, "OPEN"),
            PortStatus::Closed => write!(formatter, "CLOSED"),
            PortStatus::Timeout => write!(formatter, "TIMEOUT"),
            PortStatus::Error => write!(formatter, "ERROR"),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ScanResult {
    pub port: u16,
    pub status: PortStatus,
    pub response_time_ms: Option<u64>,
}

pub fn scan_server(server: &Server) -> Vec<ScanResult> {
    let mut results = Vec::new();

    for port in &server.ports {
        let result = scan_tcp_port(&server.address, *port, server.timeout_ms);
        results.push(result);
    }

    results
}

fn scan_tcp_port(address: &str, port: u16, timeout_ms: u64) -> ScanResult {
    let target = format!("{}:{}", address, port);
    let timeout = Duration::from_millis(timeout_ms);
    let start_time = Instant::now();

    let socket_address = match target.to_socket_addrs() {
        Ok(mut addresses) => match addresses.next() {
            Some(address) => address,
            None => {
                return ScanResult {
                    port,
                    status: PortStatus::Error,
                    response_time_ms: None,
                };
            }
        },
        Err(_) => {
            return ScanResult {
                port,
                status: PortStatus::Error,
                response_time_ms: None,
            };
        }
    };

    match TcpStream::connect_timeout(&socket_address, timeout) {
        Ok(_) => ScanResult {
            port,
            status: PortStatus::Open,
            response_time_ms: Some(start_time.elapsed().as_millis() as u64),
        },
        Err(error) => {
            let status = if error.kind() == std::io::ErrorKind::TimedOut {
                PortStatus::Timeout
            } else {
                PortStatus::Closed
            };

            ScanResult {
                port,
                status,
                response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_status_display_should_return_correct_text() {
        assert_eq!(PortStatus::Open.to_string(), "OPEN");
        assert_eq!(PortStatus::Closed.to_string(), "CLOSED");
        assert_eq!(PortStatus::Timeout.to_string(), "TIMEOUT");
        assert_eq!(PortStatus::Error.to_string(), "ERROR");
    }
}
