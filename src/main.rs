mod config;
mod output;
mod report;
mod scanner;

use config::load_config;
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

    let mut report_content = create_report_header();

    for server in &config.servers {
        match server.protocol.as_str() {
            "tcp" => {
                let results = scan_server(server);

                print_server_dashboard(server, &results);
                add_server_report(&mut report_content, server, &results);
            }
            _ => {
                print_unsupported_protocol(server);
                add_unsupported_protocol_report(&mut report_content, server);
            }
        }
    }

    match save_report("scan_report.txt", &report_content) {
        Ok(()) => println!("Report gespeichert: scan_report.txt"),
        Err(error) => println!("Fehler beim Speichern des Reports: {}", error),
    }
}
