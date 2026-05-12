# Project Summary: Embedded Network Monitoring Framework in Rust

## 1. Projektziel

Das Ziel des Projekts ist die Entwicklung eines kleinen Netzwerkdiagnose-Tools in Rust.

Das Tool liest Server, IP-Adressen, Ports und Timeout-Werte aus einer JSON-Konfigurationsdatei ein. Danach prüft es, ob die angegebenen TCP-Ports erreichbar sind, misst die Antwortzeit und erzeugt eine strukturierte Ausgabe.

Das Projekt ist nicht als Hacking-Tool gedacht, sondern als legitimes Diagnosewerkzeug für Embedded Systems, TCP/IP-Grundlagen und Netzwerküberwachung.

## 2. Aktuelle Funktionen

Das Projekt unterstützt aktuell:

- Einlesen einer config.json
- mehrere Server
- mehrere TCP-Ports pro Server
- konfigurierbarer Timeout pro Server
- TCP-Verbindungsprüfung
- Antwortzeitmessung pro Port
- Gesamt-Scan-Dauer pro Server
- parallele TCP-Port-Scans
- farbige Terminalausgabe
- TXT-Report: scan_report.txt
- JSON-Report: scan_report.json
- bessere Fehlerklassifikation
- Unit-Tests mit cargo test
- Git-Versionierung

## 3. Unterstützte Statuswerte

Das Tool unterscheidet aktuell folgende Statuswerte:

- OPEN
- CLOSED
- TIMEOUT
- NETWORK_UNREACHABLE
- HOST_UNREACHABLE
- DNS_ERROR
- PERMISSION_DENIED
- ERROR

Dadurch ist die Diagnose genauer als bei einem einfachen Offen/Geschlossen-Test.

## 4. Projektstruktur

src/main.rs

Steuert den gesamten Programmablauf:
Konfiguration laden, Server durchgehen, Scan starten, Ergebnisse ausgeben und Reports speichern.

src/config.rs

Lädt die JSON-Konfiguration und definiert die Strukturen Config und Server.

src/scanner.rs

Enthält die eigentliche TCP-Scanlogik.
Hier werden PortStatus und ScanResult definiert.
Außerdem werden die Ports parallel mit Threads geprüft.

src/output.rs

Erzeugt die farbige Terminalausgabe mit Tabelle, Status, Antwortzeit, Gesamtzeit und Zusammenfassung.

src/report.rs

Erzeugt den menschenlesbaren Textreport scan_report.txt.

src/json_report.rs

Erzeugt den maschinenlesbaren JSON-Report scan_report.json.

## 5. Wichtige Rust-Konzepte

In diesem Projekt werden viele wichtige Rust-Konzepte praktisch verwendet:

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
- impl Display
- Datei-I/O mit fs::read_to_string und fs::write
- JSON-Verarbeitung mit serde und serde_json
- Serialize und Deserialize
- Threads mit std::thread
- thread::spawn
- move-Closure
- JoinHandle
- join()
- clone()
- sort_by_key()
- Zeitmessung mit Instant
- Unit-Tests mit #[test]

## 6. Warum Rust?

Rust ist für dieses Projekt passend, weil es moderne Systemprogrammierung ermöglicht.

Wichtige Vorteile gegenüber C:

- stärkere Typsicherheit
- sichere Fehlerbehandlung mit Result und Option
- keine klassischen Null-Pointer-Probleme
- kontrollierter Umgang mit Speicher durch Ownership und Borrowing
- gute Toolchain mit cargo fmt, cargo clippy und cargo test
- geeignet für Systemprogrammierung und Embedded-nahe Anwendungen

## 7. Demo-Befehle

Code formatieren:

cargo fmt

Code prüfen:

cargo clippy

Tests ausführen:

cargo test

Programm starten:

cargo run

Textreport anzeigen:

cat scan_report.txt

JSON-Report anzeigen:

cat scan_report.json

Git-Status prüfen:

git status

## 8. Beispiel-Demo

Ein Testserver kann lokal gestartet werden mit:

python3 -m http.server 8080 --bind 127.0.0.1

Danach sollte das Rust-Tool bei 127.0.0.1 Port 8080 den Status OPEN anzeigen.

Wenn der Python-Server mit Strg + C beendet wird, sollte das Tool bei Port 8080 wieder CLOSED anzeigen.

Damit kann gezeigt werden, dass das Tool echte Netzwerkzustände prüft und nicht nur feste Werte ausgibt.

## 9. Parallele Scans

Die Ports eines Servers werden parallel geprüft.

Ohne Parallelisierung:

Wenn 4 Ports jeweils 1000 ms Timeout haben, dauert der Scan ungefähr 4000 ms.

Mit Parallelisierung:

Die 4 Ports werden gleichzeitig geprüft. Die Gesamtdauer liegt dann ungefähr bei 1000 ms.

Dadurch ist das Tool bei mehreren Timeout-Fällen deutlich schneller.

## 10. Was ich bisher gelernt habe

Durch das Projekt habe ich gelernt:

- wie man ein Rust-Projekt modular aufbaut
- wie man JSON-Dateien einliest
- wie man eigene Datentypen mit struct und enum definiert
- wie man Fehler mit Result, Option und match behandelt
- wie man TCP-Verbindungen mit Timeout prüft
- wie man Messzeiten mit Instant erfasst
- wie man Reports als TXT und JSON erzeugt
- wie man Tests mit cargo test schreibt
- wie man Code mit cargo fmt und cargo clippy prüft
- wie man Git für Versionierung verwendet
- wie man einfache parallele Verarbeitung mit Threads umsetzt

## 11. Nächste mögliche Erweiterungen

Mögliche nächste Schritte:

- UDP-Unterstützung
- Ping/ICMP-Prüfung
- Web-Dashboard
- Logging-System
- Konfigurierbare Ausgabeformate
- Export als CSV
- bessere Kommandozeilenargumente
- Integration in Embedded-Testumgebungen
- Vergleich zwischen sequentiellem und parallelem Scanmodus

## 12. Aktueller Projektstatus

Das Projekt besitzt aktuell einen stabilen Kern und ist für eine erste Vorstellung gut geeignet.

Es zeigt wichtige Rust-Konzepte praktisch anhand eines Netzwerkdiagnose-Tools.
