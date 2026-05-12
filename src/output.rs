use crate::config::Server;
use crate::scanner::{PortStatus, ScanResult};

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

pub fn print_header() {
    println!("Embedded Network Monitoring Framework");
    println!("====================================\n");
}

pub fn print_server_dashboard(server: &Server, results: &[ScanResult], total_duration_ms: u64) {
    print_server_info(server);

    println!("{:<10}{:<22}Zeit", "Port", "Status");
    println!("----------------------------------------");

    for result in results {
        let status_text = result.status.to_string();
        let status_column = format!("{:<22}", status_text);
        let colored_status = color_status(&result.status, &status_column);

        let response_time = match result.response_time_ms {
            Some(time) => format!("{} ms", time),
            None => String::from("-"),
        };

        println!("{:<10}{}{}", result.port, colored_status, response_time);
    }

    println!("\nScan-Dauer gesamt: {} ms", total_duration_ms);

    print_summary(results);

    println!();
}

pub fn print_unsupported_protocol(server: &Server) {
    print_server_info(server);

    println!(
        "Protokoll '{}' wird aktuell nicht unterstützt.",
        server.protocol
    );
    println!("Aktuell implementiert: tcp\n");
}

fn print_server_info(server: &Server) {
    println!("Server: {}", server.name);
    println!("Adresse: {}", server.address);
    println!("Protokoll: {}", server.protocol);
    println!("Timeout: {} ms\n", server.timeout_ms);
}

fn print_summary(results: &[ScanResult]) {
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

    println!("\nZusammenfassung:");
    println!("{}OPEN:{} {}", GREEN, RESET, open_count);
    println!("{}CLOSED:{} {}", RED, RESET, closed_count);
    println!("{}TIMEOUT:{} {}", YELLOW, RESET, timeout_count);
    println!(
        "{}NETWORK_UNREACHABLE:{} {}",
        CYAN, RESET, network_unreachable_count
    );
    println!(
        "{}HOST_UNREACHABLE:{} {}",
        CYAN, RESET, host_unreachable_count
    );
    println!("{}DNS_ERROR:{} {}", MAGENTA, RESET, dns_error_count);
    println!(
        "{}PERMISSION_DENIED:{} {}",
        MAGENTA, RESET, permission_denied_count
    );
    println!("{}ERROR:{} {}", MAGENTA, RESET, error_count);
}

fn color_status(status: &PortStatus, text: &str) -> String {
    match status {
        PortStatus::Open => format!("{}{}{}", GREEN, text, RESET),
        PortStatus::Closed => format!("{}{}{}", RED, text, RESET),
        PortStatus::Timeout => format!("{}{}{}", YELLOW, text, RESET),
        PortStatus::NetworkUnreachable => format!("{}{}{}", CYAN, text, RESET),
        PortStatus::HostUnreachable => format!("{}{}{}", CYAN, text, RESET),
        PortStatus::DnsError => format!("{}{}{}", MAGENTA, text, RESET),
        PortStatus::PermissionDenied => format!("{}{}{}", MAGENTA, text, RESET),
        PortStatus::Error => format!("{}{}{}", MAGENTA, text, RESET),
    }
}
