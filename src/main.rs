mod config;
mod csv_report;
mod json_report;
mod output;
mod report;
mod scanner;

use config::load_config;
use csv_report::{
    add_server_csv_report, add_unsupported_protocol_csv_report, create_csv_report_header,
    save_csv_report,
};
use json_report::{
    add_server_json_report, add_unsupported_protocol_json_report, create_json_report,
    save_json_report,
};
use output::{print_header, print_server_dashboard, print_unsupported_protocol};
use report::{
    add_server_report, add_unsupported_protocol_report, create_report_header, save_report,
};
use scanner::{scan_tcp_server, scan_udp_server};
use std::time::Instant;

fn main() {
    print_header();

    let config = match load_config("config.json") {
        Ok(config) => config,
        Err(error) => {
            println!("Fehler: config.json konnte nicht geladen werden.");
            println!("Details: {}", error);
            return;
        }
    };

    let mut text_report_content = create_report_header();
    let mut json_report_content = create_json_report();
    let mut csv_report_content = create_csv_report_header();

    for server in &config.servers {
        match server.protocol.as_str() {
            "tcp" => {
                let scan_start = Instant::now();
                let results = scan_tcp_server(server);
                let total_duration_ms = scan_start.elapsed().as_millis() as u64;

                print_server_dashboard(server, &results, total_duration_ms);

                add_server_report(
                    &mut text_report_content,
                    server,
                    &results,
                    total_duration_ms,
                );

                add_server_json_report(
                    &mut json_report_content,
                    server,
                    results.clone(),
                    total_duration_ms,
                );

                add_server_csv_report(&mut csv_report_content, server, &results, total_duration_ms);
            }
            "udp" => {
                let scan_start = Instant::now();
                let results = scan_udp_server(server);
                let total_duration_ms = scan_start.elapsed().as_millis() as u64;

                print_server_dashboard(server, &results, total_duration_ms);

                add_server_report(
                    &mut text_report_content,
                    server,
                    &results,
                    total_duration_ms,
                );

                add_server_json_report(
                    &mut json_report_content,
                    server,
                    results.clone(),
                    total_duration_ms,
                );

                add_server_csv_report(&mut csv_report_content, server, &results, total_duration_ms);
            }
            _ => {
                print_unsupported_protocol(server);
                add_unsupported_protocol_report(&mut text_report_content, server);
                add_unsupported_protocol_json_report(&mut json_report_content, server);
                add_unsupported_protocol_csv_report(&mut csv_report_content, server);
            }
        }
    }

    match save_report("scan_report.txt", &text_report_content) {
        Ok(()) => println!("TXT-Report gespeichert: scan_report.txt"),
        Err(error) => println!("Fehler beim Speichern des TXT-Reports: {}", error),
    }

    match save_json_report("scan_report.json", &json_report_content) {
        Ok(()) => println!("JSON-Report gespeichert: scan_report.json"),
        Err(error) => println!("Fehler beim Speichern des JSON-Reports: {}", error),
    }

    match save_csv_report("scan_report.csv", &csv_report_content) {
        Ok(()) => println!("CSV-Report gespeichert: scan_report.csv"),
        Err(error) => println!("Fehler beim Speichern des CSV-Reports: {}", error),
    }
}
