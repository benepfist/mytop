# Spec: Top-Ansicht (GetData)

## Zweck
Zeigt Server-Kopfmetrik und Prozessliste im Stil von `top`.

## Datenquellen
- `SHOW GLOBAL STATUS`
- `SHOW FULL PROCESSLIST`

## Verhalten Header
- Berechnet Uptime, Gesamt-QPS, aktuelle QPS, Slow-Query-Rate.
- Berechnet Query-Typ-Verteilung (Select/Insert/Update/Delete).
- Berechnet Key-Cache-Effizienz.
- Zeigt Query-Cache-Hits nur wenn Query Cache verfügbar ist.
- Nutzt bei vorhandener `Time::HiRes` hochauflösende Zeitdeltas.

## Verhalten Threadliste
- Dynamische Breite abhängig von Terminalgröße.
- Hostnormalisierung: Domainkürzung oder optionales Reverse-DNS (`resolve`).
- Sortierung nach `Time` (normal/reverse via `sort`).
- Filterung nach User/DB/Host.
- Optionales Ausblenden von Idle/Sleep/Binlog-Dump Threads.
- Farbcodierung nach Command (`Query`, `Sleep`, `Connect`) bei Farbunterstützung.
- Cacht Query-, DB- und User-Infos pro Thread-ID für Folgefunktionen.
