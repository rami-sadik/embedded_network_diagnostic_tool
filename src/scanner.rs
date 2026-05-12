use crate::config::Server;
use serde::Serialize;
use std::fmt;
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Clone, Copy)]
pub enum PortStatus {
    #[serde(rename = "OPEN")]
    Open,

    #[serde(rename = "CLOSED")]
    Closed,

    #[serde(rename = "TIMEOUT")]
    Timeout,

    #[serde(rename = "NETWORK_UNREACHABLE")]
    NetworkUnreachable,

    #[serde(rename = "HOST_UNREACHABLE")]
    HostUnreachable,

    #[serde(rename = "DNS_ERROR")]
    DnsError,

    #[serde(rename = "PERMISSION_DENIED")]
    PermissionDenied,

    #[serde(rename = "ERROR")]
    Error,
}

impl fmt::Display for PortStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortStatus::Open => write!(formatter, "OPEN"),
            PortStatus::Closed => write!(formatter, "CLOSED"),
            PortStatus::Timeout => write!(formatter, "TIMEOUT"),
            PortStatus::NetworkUnreachable => write!(formatter, "NETWORK_UNREACHABLE"),
            PortStatus::HostUnreachable => write!(formatter, "HOST_UNREACHABLE"),
            PortStatus::DnsError => write!(formatter, "DNS_ERROR"),
            PortStatus::PermissionDenied => write!(formatter, "PERMISSION_DENIED"),
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
    let mut handles = Vec::new();

    for port in &server.ports {
        let address = server.address.clone();
        let timeout_ms = server.timeout_ms;
        let port = *port;

        let handle = thread::spawn(move || scan_tcp_port(&address, port, timeout_ms));

        handles.push((port, handle));
    }

    let mut results = Vec::new();

    for (port, handle) in handles {
        match handle.join() {
            Ok(result) => results.push(result),
            Err(_) => results.push(ScanResult {
                port,
                status: PortStatus::Error,
                response_time_ms: None,
            }),
        }
    }

    results.sort_by_key(|result| result.port);

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
                    status: PortStatus::DnsError,
                    response_time_ms: None,
                };
            }
        },
        Err(_) => {
            return ScanResult {
                port,
                status: PortStatus::DnsError,
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
            let status = match error.kind() {
                std::io::ErrorKind::ConnectionRefused => PortStatus::Closed,
                std::io::ErrorKind::TimedOut => PortStatus::Timeout,
                std::io::ErrorKind::NetworkUnreachable => PortStatus::NetworkUnreachable,
                std::io::ErrorKind::HostUnreachable => PortStatus::HostUnreachable,
                std::io::ErrorKind::PermissionDenied => PortStatus::PermissionDenied,
                _ => PortStatus::Error,
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
        assert_eq!(
            PortStatus::NetworkUnreachable.to_string(),
            "NETWORK_UNREACHABLE"
        );
        assert_eq!(PortStatus::HostUnreachable.to_string(), "HOST_UNREACHABLE");
        assert_eq!(PortStatus::DnsError.to_string(), "DNS_ERROR");
        assert_eq!(
            PortStatus::PermissionDenied.to_string(),
            "PERMISSION_DENIED"
        );
        assert_eq!(PortStatus::Error.to_string(), "ERROR");
    }
}
