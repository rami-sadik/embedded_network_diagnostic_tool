use crate::config::Server;
use crate::ping::PingResult;
use crate::scanner::ScanResult;
use std::error::Error;
use std::fs;

pub fn create_csv_report_header() -> String {
    String::from(
        "server_name,address,protocol,port,status,response_time_ms,total_duration_ms,message\n",
    )
}

pub fn add_server_csv_report(
    report: &mut String,
    server: &Server,
    results: &[ScanResult],
    total_duration_ms: u64,
) {
    for result in results {
        let response_time = match result.response_time_ms {
            Some(time) => time.to_string(),
            None => String::new(),
        };

        report.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            escape_csv_value(&server.name),
            escape_csv_value(&server.address),
            escape_csv_value(&server.protocol),
            result.port,
            result.status,
            response_time,
            total_duration_ms,
            ""
        ));
    }
}

pub fn add_ping_csv_report(
    report: &mut String,
    server: &Server,
    result: &PingResult,
    total_duration_ms: u64,
) {
    let response_time = match result.response_time_ms {
        Some(time) => time.to_string(),
        None => String::new(),
    };

    report.push_str(&format!(
        "{},{},{},{},{},{},{},{}\n",
        escape_csv_value(&server.name),
        escape_csv_value(&server.address),
        escape_csv_value(&server.protocol),
        "",
        result.status,
        response_time,
        total_duration_ms,
        "Ping host check"
    ));
}

pub fn add_unsupported_protocol_csv_report(report: &mut String, server: &Server) {
    report.push_str(&format!(
        "{},{},{},{},{},{},{},{}\n",
        escape_csv_value(&server.name),
        escape_csv_value(&server.address),
        escape_csv_value(&server.protocol),
        "",
        "UNSUPPORTED_PROTOCOL",
        "",
        "",
        "Protocol is currently not supported"
    ));
}

pub fn save_csv_report(path: &str, report: &str) -> Result<(), Box<dyn Error>> {
    fs::write(path, report)?;
    Ok(())
}

fn escape_csv_value(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{PortStatus, ScanResult};

    #[test]
    fn csv_report_should_contain_scan_result() {
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

        let mut report = create_csv_report_header();
        add_server_csv_report(&mut report, &server, &results, 7);

        assert!(report.contains("Test Server"));
        assert!(report.contains("127.0.0.1"));
        assert!(report.contains("tcp"));
        assert!(report.contains("8080"));
        assert!(report.contains("OPEN"));
        assert!(report.contains("5"));
        assert!(report.contains("7"));
    }
}
