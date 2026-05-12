mod config;
mod json_report;
mod output;
mod report;
mod scanner;

use config::load_config;
use json_report::{
    add_server_json_report, add_unsupported_protocol_json_report, create_json_report,
    save_json_report,
};
use output::{print_header, print_server_dashboard, print_unsupported_protocol};
use report::{
    add_server_report, add_unsupported_protocol_report, create_report_header, save_report,
};
use scanner::scan_server;

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

    for server in &config.servers {
        match server.protocol.as_str() {
            "tcp" => {
                let results = scan_server(server);

                print_server_dashboard(server, &results);
                add_server_report(&mut text_report_content, server, &results);
                add_server_json_report(&mut json_report_content, server, results.clone());
            }
            _ => {
                print_unsupported_protocol(server);
                add_unsupported_protocol_report(&mut text_report_content, server);
                add_unsupported_protocol_json_report(&mut json_report_content, server);
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
}
