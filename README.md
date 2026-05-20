# Embedded Network Diagnostic Tool

## Overview

This project is a small network diagnostic tool written in Rust.

It is designed for learning and demonstrating Embedded Linux, TCP/IP basics, Rust system programming and simple network diagnostics on a Raspberry Pi.

The tool reads a JSON configuration file, checks configured hosts and ports, measures response times, classifies network results and generates reports in TXT, JSON and CSV format.

This project is intended for authorized diagnostics in private or laboratory networks only. It is not designed for unauthorized scanning or offensive security use.

## Main Features

- Read targets from a JSON configuration file
- Scan multiple hosts
- Scan multiple ports per host
- Configurable timeout per host
- TCP port diagnostics
- UDP support for selected test scenarios
- Host reachability check / ping result
- Response time measurement in milliseconds
- Total scan duration per host
- Structured terminal output
- TXT report generation
- JSON report generation
- CSV report generation
- Unit tests with cargo test
- Code quality checks with cargo fmt and cargo clippy
- Raspberry Pi deployment
- systemd timer automation
- Simple web dashboard using generated JSON data

## Status Classification

The tool can classify results with the following status values:

- OPEN
- CLOSED
- TIMEOUT
- NETWORK_UNREACHABLE
- HOST_UNREACHABLE
- DNS_ERROR
- PERMISSION_DENIED
- ERROR

This makes the tool more useful than a simple open/closed port checker.

## Architecture

Raspberry Pi / Linux system
  -> Rust diagnostic tool
  -> config.json
  -> TCP/UDP diagnostics
  -> response time measurement
  -> result classification
  -> scan_report.txt
  -> scan_report.json
  -> scan_report.csv
  -> optional nginx web dashboard

## Project Structure

embedded_network_diagnostic_tool/
  Cargo.toml
  Cargo.lock
  README.md
  PROJECT_SUMMARY.md
  config.json
  config.local-test.json
  run_scan_dashboard.sh
  udp_echo_server.py

  src/
    main.rs
    cli.rs
    config.rs
    scanner.rs
    ping.rs
    output.rs
    report.rs
    json_report.rs
    csv_report.rs

  examples/
    example_config.local-test.json
    example_scan_report.txt
    example_scan_report.json
    example_scan_report.csv

## Configuration Example

Example config.json:

{
  "servers": [
    {
      "name": "Local Test Server",
      "address": "127.0.0.1",
      "protocol": "tcp",
      "timeout_ms": 1000,
      "ports": [22, 80, 443, 8080]
    }
  ]
}

Each server entry contains:

- name: human-readable device name
- address: IP address or hostname
- protocol: tcp or udp
- timeout_ms: timeout in milliseconds
- ports: list of ports to check

## Usage

Run with default configuration:

cargo run

Run with a custom configuration file:

cargo run -- --config config.local-test.json

Show help:

cargo run -- --help

Build release version:

cargo build --release

Run release binary:

./target/release/embedded_network_diagnostic_tool

## Reports

After each run, the tool generates:

- scan_report.txt
- scan_report.json
- scan_report.csv

The TXT report is intended for humans.

The JSON report is intended for dashboards, automation or further processing.

The CSV report can be opened in spreadsheet tools.

Example reports are stored in the examples/ directory.

Generated live reports are ignored by Git because they change after every scan.

## Raspberry Pi Deployment

On the Raspberry Pi, the release binary and runtime files can be installed into:

/opt/home-network-monitor

Example runtime files:

- embedded_network_diagnostic_tool
- config.json
- discover_and_scan.sh
- scan_report.txt
- scan_report.json
- scan_report.csv

A systemd timer can run the diagnostic script automatically every few minutes.

Useful commands:

systemctl status home-network-monitor.timer --no-pager
systemctl status home-network-monitor.service --no-pager
journalctl -u home-network-monitor.service -n 50 --no-pager

## Web Dashboard

The dashboard can be served with nginx from:

/var/www/html/net-dashboard

Typical dashboard files:

- index.html
- style.css
- app.js
- results.json

The dashboard reads results.json and displays the latest diagnostic results in the browser.

Example local URL:

http://<raspberry-pi-ip>/net-dashboard/

## Testing and Code Quality

Format code:

cargo fmt

Check formatting:

cargo fmt --check

Run Clippy:

cargo clippy

Run tests:

cargo test

The project contains unit tests for CLI parsing, report generation, JSON output, CSV output, ping status handling and scan status formatting.

## Rust Concepts Used

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
- JSON parsing and serialization with serde
- time measurement with Instant
- threads with std::thread
- unit tests with #[test]
- Cargo workflow with fmt, clippy and test

## Embedded Systems Relevance

The project is relevant for Embedded Systems because many embedded Linux devices, industrial controllers, gateways and Raspberry Pi based systems need simple diagnostic tools to verify network availability.

Typical use cases:

- Check whether an embedded Linux device is reachable
- Verify whether SSH, HTTP or other services are available
- Measure response times in a local lab network
- Generate diagnostic reports
- Display results on a lightweight dashboard

## Safety and Scope

This tool must only be used in networks where the user has permission to perform diagnostics.

Recommended scope:

- private home network
- university lab network with permission
- own Raspberry Pi
- own embedded devices
- local test environment

Not intended scope:

- unauthorized public IP scanning
- attacking systems
- bypassing security mechanisms
- offensive security operations

## Current Status

The project has a stable Rust core, working Raspberry Pi deployment, generated TXT/JSON/CSV reports, unit tests and a simple dashboard integration.

It is suitable as a small 2 ECTS Rust project and as a portfolio project for Embedded Systems, Linux and network diagnostics.

## Possible Future Extensions

- STM32 integration for hardware status LEDs
- Serial communication between Raspberry Pi and STM32
- More detailed dashboard visualization
- Configurable scan intervals
- Better device naming
- Export of historical scan results
- Packaging as a system service
