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

## Neue Erweiterungen

### TXT-Report

Nach jedem Programmstart wird ein Textreport erzeugt:

scan_report.txt

Dieser Report ist für Menschen gut lesbar und enthält Serverinformationen, Portstatus, Antwortzeiten und eine Zusammenfassung.

### JSON-Report

Zusätzlich wird ein JSON-Report erzeugt:

scan_report.json

Dieser Report ist maschinenlesbar und kann später für weitere Tools, ein Web-Dashboard oder automatische Auswertung verwendet werden.

Für den JSON-Report wird serde Serialize verwendet.

### Farbige Terminalausgabe

Die Statuswerte werden im Terminal farbig dargestellt:

- OPEN: grün
- CLOSED: rot
- TIMEOUT: gelb
- ERROR: magenta

Die Farben werden mit ANSI-Escape-Codes umgesetzt. Dafür wird keine zusätzliche externe Bibliothek benötigt.

## Aktuelle Befehle

Programm starten:

cargo run

Tests ausführen:

cargo test

Code formatieren:

cargo fmt

Code prüfen:

cargo clippy

TXT-Report anzeigen:

cat scan_report.txt

JSON-Report anzeigen:

cat scan_report.json

## Aktueller Entwicklungsstand

Das Projekt unterstützt aktuell:

- JSON-Konfiguration
- mehrere Server
- mehrere TCP-Ports pro Server
- Timeout pro Server
- Antwortzeitmessung
- Statusausgabe mit OPEN, CLOSED, TIMEOUT und ERROR
- farbige Terminalausgabe
- TXT-Report
- JSON-Report
- Unit-Tests
- Git-Versionierung


## Erweiterungen: Fehlerklassifikation, Parallelisierung und Scan-Dauer

### Bessere Fehlerklassifikation

Das Tool unterscheidet inzwischen mehrere Statuswerte:

- OPEN
- CLOSED
- TIMEOUT
- NETWORK_UNREACHABLE
- HOST_UNREACHABLE
- DNS_ERROR
- PERMISSION_DENIED
- ERROR

Dadurch ist die Diagnose genauer als bei einer einfachen Offen/Geschlossen-Prüfung.

### Parallele TCP-Port-Scans

Die Ports eines Servers werden parallel geprüft.  
Dafür wird in Rust `std::thread::spawn` verwendet.

Vorteil:

Wenn mehrere Ports in einen Timeout laufen, muss das Programm nicht jeden Timeout nacheinander abwarten.  
Dadurch wird der gesamte Scan schneller.

Beispiel:

Ohne Parallelisierung:

4 Timeout-Ports mit jeweils 1000 ms Timeout dauern ungefähr 4000 ms.

Mit Parallelisierung:

4 Timeout-Ports mit jeweils 1000 ms Timeout dauern ungefähr 1000 ms.

### Gesamt-Scan-Dauer pro Server

Zusätzlich zur Antwortzeit pro Port misst das Programm auch die Gesamtdauer des Scans pro Server.

Diese Gesamtzeit wird angezeigt in:

- Terminalausgabe
- scan_report.txt
- scan_report.json

Dadurch kann man gut erkennen, welchen Vorteil die parallele Scanlogik bringt.

## Aktuelle Rust-Konzepte im Projekt

Zusätzlich zu den bisherigen Rust-Konzepten werden jetzt auch verwendet:

- std::thread
- thread::spawn
- move-Closure
- JoinHandle
- join()
- clone()
- sort_by_key()
- Zeitmessung mit Instant

