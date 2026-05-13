use crate::config::Server;
use crate::ping::PingResult;
use crate::scanner::ScanResult;
use serde::Serialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize)]
pub struct JsonReport {
    tool: String,
    servers: Vec<JsonServerReport>,
}

#[derive(Debug, Serialize)]
struct JsonServerReport {
    name: String,
    address: String,
    protocol: String,
    timeout_ms: u64,
    total_duration_ms: Option<u64>,
    message: Option<String>,
    results: Vec<ScanResult>,
    ping: Option<PingResult>,
}

pub fn create_json_report() -> JsonReport {
    JsonReport {
        tool: String::from("Embedded Network Monitoring Framework"),
        servers: Vec::new(),
    }
}

pub fn add_server_json_report(
    report: &mut JsonReport,
    server: &Server,
    results: Vec<ScanResult>,
    total_duration_ms: u64,
) {
    let server_report = JsonServerReport {
        name: server.name.clone(),
        address: server.address.clone(),
        protocol: server.protocol.clone(),
        timeout_ms: server.timeout_ms,
        total_duration_ms: Some(total_duration_ms),
        message: None,
        results,
        ping: None,
    };

    report.servers.push(server_report);
}

pub fn add_ping_json_report(
    report: &mut JsonReport,
    server: &Server,
    ping_result: PingResult,
    total_duration_ms: u64,
) {
    let server_report = JsonServerReport {
        name: server.name.clone(),
        address: server.address.clone(),
        protocol: server.protocol.clone(),
        timeout_ms: server.timeout_ms,
        total_duration_ms: Some(total_duration_ms),
        message: None,
        results: Vec::new(),
        ping: Some(ping_result),
    };

    report.servers.push(server_report);
}

pub fn add_unsupported_protocol_json_report(report: &mut JsonReport, server: &Server) {
    let server_report = JsonServerReport {
        name: server.name.clone(),
        address: server.address.clone(),
        protocol: server.protocol.clone(),
        timeout_ms: server.timeout_ms,
        total_duration_ms: None,
        message: Some(String::from("Protocol is currently not supported")),
        results: Vec::new(),
        ping: None,
    };

    report.servers.push(server_report);
}

pub fn save_json_report(path: &str, report: &JsonReport) -> Result<(), Box<dyn Error>> {
    let json_content = serde_json::to_string_pretty(report)?;
    fs::write(path, json_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{PortStatus, ScanResult};

    #[test]
    fn json_report_should_contain_server_result() {
        let server = Server {
            name: String::from("Test Server"),
            address: String::from("127.0.0.1"),
            protocol: String::from("tcp"),
            timeout_ms: 1000,
            ports: vec![8080],
        };

        let results = vec![ScanResult {
            port: 8080,
            status: PortStatus::Open,
            response_time_ms: Some(5),
        }];

        let mut report = create_json_report();
        add_server_json_report(&mut report, &server, results, 7);

        let json = serde_json::to_string_pretty(&report).unwrap();

        assert!(json.contains("Test Server"));
        assert!(json.contains("127.0.0.1"));
        assert!(json.contains("8080"));
        assert!(json.contains("OPEN"));
        assert!(json.contains("total_duration_ms"));
        assert!(json.contains("7"));
    }
}
