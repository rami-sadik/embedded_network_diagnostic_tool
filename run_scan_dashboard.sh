#!/bin/bash

cd /home/rami/rust-projekte/embedded_network_diagnostic_tool || exit 1

echo "Starte Heimnetz-Scan..."
cargo run

echo "Kopiere Ergebnis ins nginx-Dashboard..."
cp scan_report.json /var/www/html/net-dashboard/results.json

echo "Fertig. Dashboard:"
echo "http://192.168.178.107/net-dashboard/"
