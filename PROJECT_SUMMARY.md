# Project Summary: Embedded Network Diagnostic Tool

## 1. Project Goal

The goal of this project is to develop a small network diagnostic tool in Rust.

The tool reads hosts, IP addresses, protocols, ports and timeout values from a JSON configuration file. It then checks the configured network targets, measures response times, classifies the results and generates structured reports.

The project is intended as a legitimate diagnostic tool for Embedded Systems, Embedded Linux, TCP/IP basics and Rust system programming. It is not intended for unauthorized scanning or offensive security use.

## 2. Current Features

The project currently supports:

- JSON-based configuration
- multiple hosts
- multiple ports per host
- configurable timeout per host
- TCP port diagnostics
- UDP support for selected test scenarios
- host reachability / ping result
- response time measurement in milliseconds
- total scan duration per host
- structured terminal output
- TXT report generation
- JSON report generation
- CSV report generation
- improved result classification
- command-line argument for custom config files
- unit tests with cargo test
- code checks with cargo fmt and cargo clippy
- Raspberry Pi deployment
- systemd timer automation
- simple web dashboard integration

## 3. Result Classification

The tool distinguishes between the following result states:

- OPEN
- CLOSED
- TIMEOUT
- NETWORK_UNREACHABLE
- HOST_UNREACHABLE
- DNS_ERROR
- PERMISSION_DENIED
- ERROR

## 4. Project Structure

Important files and modules:

- Cargo.toml: Rust project configuration
- README.md: GitHub documentation
- PROJECT_SUMMARY.md: project summary
- config.json: default configuration
- config.local-test.json: local test configuration
- run_scan_dashboard.sh: helper script for dashboard updates
- udp_echo_server.py: small UDP test helper
- examples/: example configuration and example reports

Rust source modules:

- src/main.rs: controls the main program flow
- src/cli.rs: parses command-line arguments
- src/config.rs: loads and represents the JSON configuration
- src/scanner.rs: contains TCP/UDP scan logic and result types
- src/ping.rs: checks host reachability
- src/output.rs: creates terminal output
- src/report.rs: creates the TXT report
- src/json_report.rs: creates the JSON report
- src/csv_report.rs: creates the CSV report

## 5. Rust Concepts Used

This project demonstrates several important Rust concepts:

- struct
- enum
- Vec
- String
- Result
- Option
- match
- modules with mod
- public functions with pub
- references and borrowing
- error handling with ?
- impl Display
- file I/O
- JSON parsing with serde
- JSON serialization with serde_json
- time measurement with Instant
- threads with std::thread
- unit tests with #[test]
- Cargo workflow with fmt, clippy and test

## 6. Why Rust?

Rust is suitable for this project because it supports modern systems programming with strong safety guarantees.

Important advantages compared with C:

- stronger type safety
- structured error handling with Result and Option
- no classic null pointer problems
- memory safety through ownership and borrowing
- modern tooling with cargo fmt, cargo clippy and cargo test
- good fit for Embedded Linux and system-level tools

## 7. Raspberry Pi Deployment

The tool was deployed on a Raspberry Pi running Ubuntu Server.

Runtime directory:

/opt/home-network-monitor

Typical runtime files:

- embedded_network_diagnostic_tool
- config.json
- discover_and_scan.sh
- scan_report.txt
- scan_report.json
- scan_report.csv

A systemd timer runs the diagnostic script automatically every few minutes and updates the dashboard data.

Useful systemd commands:

- systemctl status home-network-monitor.timer --no-pager
- systemctl status home-network-monitor.service --no-pager
- journalctl -u home-network-monitor.service -n 50 --no-pager

## 8. Web Dashboard

A simple dashboard is served with nginx from:

/var/www/html/net-dashboard

The dashboard contains:

- index.html
- style.css
- app.js
- results.json

The dashboard reads results.json and displays the latest scan results in the browser.

## 9. Testing

The project contains unit tests for:

- command-line argument parsing
- scan status formatting
- ping status formatting
- timeout conversion
- TXT report generation
- JSON report generation
- CSV report generation

Quality checks:

- cargo fmt --check
- cargo clippy
- cargo test

At the current stage, all tests pass successfully.

## 10. Embedded Systems Relevance

The project is relevant for Embedded Systems because many embedded Linux devices, gateways, Raspberry Pi systems and industrial controllers need simple diagnostic tools to verify network availability.

Typical use cases:

- checking whether an embedded device is reachable
- verifying whether SSH, HTTP or other services are available
- measuring response times in a local lab network
- generating reports for debugging and documentation
- displaying device status on a lightweight dashboard

## 11. Safety and Scope

The tool must only be used in networks where the user has permission to perform diagnostics.

Allowed and intended use cases:

- private home network
- university lab network with permission
- own Raspberry Pi
- own embedded devices
- local test environment

Not intended:

- unauthorized public IP scanning
- attacking systems
- bypassing security mechanisms
- offensive security operations

## 12. Current Project Status

The project has a stable Rust core, working Raspberry Pi deployment, generated TXT/JSON/CSV reports, unit tests and a simple web dashboard integration.

It is suitable as a small 2 ECTS Rust project and as a portfolio project for Embedded Systems, Linux and network diagnostics.

## 13. Possible Future Extensions

Possible future extensions:

- STM32 integration for hardware status LEDs
- serial communication between Raspberry Pi and STM32
- more detailed dashboard visualization
- configurable scan intervals
- better device naming
- historical result storage
- packaging as a complete Linux service
