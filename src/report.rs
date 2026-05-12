use crate::config::Server;
use crate::scanner::{PortStatus, ScanResult};
use std::error::Error;
use std::fs;

pub fn create_report_header() -> String {
    let mut report = String::new();

    report.push_str("Embedded Network Monitoring Report\n");
    report.push_str("==================================\n\n");

    report
}

pub fn add_server_report(
    report: &mut String,
    server: &Server,
    results: &[ScanResult],
    total_duration_ms: u64,
) {
    report.push_str(&format!("Server: {}\n", server.name));
    report.push_str(&format!("Adresse: {}\n", server.address));
    report.push_str(&format!("Protokoll: {}\n", server.protocol));
    report.push_str(&format!("Timeout: {} ms\n\n", server.timeout_ms));

    report.push_str("Port      Status                Zeit\n");
    report.push_str("----------------------------------------\n");

    for result in results {
        let status_text = result.status.to_string();

        let response_time = match result.response_time_ms {
            Some(time) => format!("{} ms", time),
            None => String::from("-"),
        };

        report.push_str(&format!(
            "{:<10}{:<22}{}\n",
            result.port, status_text, response_time
        ));
    }

    report.push_str(&format!("\nScan-Dauer gesamt: {} ms\n", total_duration_ms));

    add_summary_to_report(report, results);

    report.push('\n');
}

pub fn add_unsupported_protocol_report(report: &mut String, server: &Server) {
    report.push_str(&format!("Server: {}\n", server.name));
    report.push_str(&format!("Adresse: {}\n", server.address));
    report.push_str(&format!("Protokoll: {}\n", server.protocol));
    report.push_str("Status: Protokoll wird aktuell nicht unterstützt.\n\n");
}

pub fn save_report(path: &str, report: &str) -> Result<(), Box<dyn Error>> {
    fs::write(path, report)?;
    Ok(())
}

fn add_summary_to_report(report: &mut String, results: &[ScanResult]) {
    let mut open_count = 0;
    let mut closed_count = 0;
    let mut timeout_count = 0;
    let mut network_unreachable_count = 0;
    let mut host_unreachable_count = 0;
    let mut dns_error_count = 0;
    let mut permission_denied_count = 0;
    let mut error_count = 0;

    for result in results {
        match result.status {
            PortStatus::Open => open_count += 1,
            PortStatus::Closed => closed_count += 1,
            PortStatus::Timeout => timeout_count += 1,
            PortStatus::NetworkUnreachable => network_unreachable_count += 1,
            PortStatus::HostUnreachable => host_unreachable_count += 1,
            PortStatus::DnsError => dns_error_count += 1,
            PortStatus::PermissionDenied => permission_denied_count += 1,
            PortStatus::Error => error_count += 1,
        }
    }

    report.push_str("\nZusammenfassung:\n");
    report.push_str(&format!("OPEN: {}\n", open_count));
    report.push_str(&format!("CLOSED: {}\n", closed_count));
    report.push_str(&format!("TIMEOUT: {}\n", timeout_count));
    report.push_str(&format!(
        "NETWORK_UNREACHABLE: {}\n",
        network_unreachable_count
    ));
    report.push_str(&format!("HOST_UNREACHABLE: {}\n", host_unreachable_count));
    report.push_str(&format!("DNS_ERROR: {}\n", dns_error_count));
    report.push_str(&format!("PERMISSION_DENIED: {}\n", permission_denied_count));
    report.push_str(&format!("ERROR: {}\n", error_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_header_should_contain_title() {
        let report = create_report_header();

        assert!(report.contains("Embedded Network Monitoring Report"));
        assert!(report.contains("=================================="));
    }

    #[test]
    fn server_report_should_contain_scan_result() {
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

        let mut report = create_report_header();

        add_server_report(&mut report, &server, &results, 7);

        assert!(report.contains("Test Server"));
        assert!(report.contains("127.0.0.1"));
        assert!(report.contains("8080"));
        assert!(report.contains("OPEN"));
        assert!(report.contains("5 ms"));
        assert!(report.contains("Scan-Dauer gesamt: 7 ms"));
    }
}
