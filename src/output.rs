use crate::config::Server;
use crate::scanner::{PortStatus, ScanResult};

pub fn print_header() {
    println!("Embedded Network Monitoring Framework");
    println!("====================================\n");
}

pub fn print_server_dashboard(server: &Server, results: &[ScanResult]) {
    print_server_info(server);

    println!("{:<10}{:<12}Zeit", "Port", "Status");
    println!("------------------------------");

    for result in results {
        let status_text = result.status.to_string();

        let response_time = match result.response_time_ms {
            Some(time) => format!("{} ms", time),
            None => String::from("-"),
        };

        println!("{:<10}{:<12}{}", result.port, status_text, response_time);
    }

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
    let mut error_count = 0;

    for result in results {
        match result.status {
            PortStatus::Open => open_count += 1,
            PortStatus::Closed => closed_count += 1,
            PortStatus::Timeout => timeout_count += 1,
            PortStatus::Error => error_count += 1,
        }
    }

    println!("\nZusammenfassung:");
    println!("OPEN: {}", open_count);
    println!("CLOSED: {}", closed_count);
    println!("TIMEOUT: {}", timeout_count);
    println!("ERROR: {}", error_count);
}
