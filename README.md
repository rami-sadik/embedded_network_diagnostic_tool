# Embedded Network Monitoring Framework in Rust

## Projektbeschreibung

Dieses Projekt ist ein kleines Netzwerkmonitoring-Framework in Rust.

Es liest eine JSON-Konfigurationsdatei ein, prüft definierte TCP-Ports von mehreren Servern und gibt die Ergebnisse als Konsolen-Dashboard sowie als Textreport aus.

Das Projekt ist nicht als Hacking-Tool gedacht, sondern als legitimes Diagnosewerkzeug für Embedded Systems, TCP/IP-Grundlagen und Netzwerküberwachung.

## Funktionen

- Einlesen einer config.json
- Unterstützung mehrerer Server
- Prüfung mehrerer TCP-Ports pro Server
- Timeout pro Server konfigurierbar
- Messung der Antwortzeit in Millisekunden
- Statusausgabe: OPEN, CLOSED, TIMEOUT, ERROR
- Ausgabe im Terminal
- Zusammenfassung pro Server
- Speicherung der Ergebnisse in scan_report.txt
- Erste Unit-Tests mit cargo test

## Projektstruktur

embedded_network_monitoring/
├── Cargo.toml
├── config.json
├── README.md
├── scan_report.txt
└── src/
    ├── main.rs
    ├── config.rs
    ├── scanner.rs
    ├── output.rs
    └── report.rs

## Module

main.rs:
Steuert den Programmablauf.

config.rs:
Liest die JSON-Konfigurationsdatei ein und definiert die Datenstrukturen Config und Server.

scanner.rs:
Enthält die Scanlogik. Die Funktion scan_server prüft alle Ports eines Servers.

output.rs:
Erzeugt die Konsolenausgabe mit Tabelle und Zusammenfassung.

report.rs:
Erzeugt den Textreport und speichert ihn als scan_report.txt.

## Beispiel config.json

{
  "servers": [
    {
      "name": "Local Test Server",
      "address": "127.0.0.1",
      "protocol": "tcp",
      "timeout_ms": 1000,
      "ports": [8080, 22, 443]
    }
  ]
}

## Programm starten

cargo run

## Code formatieren

cargo fmt

## Code prüfen

cargo clippy

## Tests ausführen

cargo test

## Verwendete Rust-Konzepte

- struct
- enum
- Vec
- String
- Result
- Option
- match
- mod
- use
- pub
- Referenzen mit &
- veränderbare Referenzen mit &mut
- Fehlerbehandlung mit ?
- Datei-I/O
- serde und serde_json
- Unit-Tests mit #[test]
- cargo fmt
- cargo clippy
- cargo test

## Bezug zu Embedded Systems

Das Tool kann als Diagnosewerkzeug für eingebettete Systeme verwendet werden, zum Beispiel für Embedded Linux Geräte, Raspberry Pi, Industriecontroller oder Netzwerkgeräte im Labor.

## Aktueller Stand

Aktuell unterstützt das Projekt TCP-Portprüfungen. Das Protokollfeld ist vorbereitet, damit später weitere Protokolle wie UDP oder ICMP/Ping ergänzt werden können.

## Mögliche Erweiterungen

- JSON-Report
- UDP-Unterstützung
- Ping/ICMP-Prüfung
- farbige Terminalausgabe
- parallele Scans
- Web-Dashboard
- Logging
