# mytop (Go Rewrite)

`mytop` ist ein terminalbasiertes CLI-Tool zur Live-Überwachung von MySQL-Threads und Server-Statuswerten.
Dieses Repository enthält die neue Implementierung in **Go**.

## Überblick

Die Anwendung startet eine textbasierte Oberfläche (TUI), lädt Konfiguration aus Datei/Umgebung/CLI-Flags und zeigt laufende Datenbankaktivitäten in verschiedenen Modi an (z. B. `top`, `qps`, `cmd`, `innodb`, `status`).

## Voraussetzungen

- Go 1.22+
- Zugriff auf eine MySQL-Instanz

## Installation & Start

### 1) Build

```bash
go build -o mytop ./cmd/mytop
```

### 2) Ausführen (Beispiel)

```bash
./mytop -u root -h localhost -P 3306 -d test
```

## CLI-Nutzung

### Wichtige Flags

- `-u`, `--user`: Datenbankbenutzer
- `-p`, `--pass`: Passwort
- `-d`, `--db`: Standarddatenbank
- `-h`, `--host`: MySQL-Host
- `-P`: MySQL-Port
- `-S`: Unix-Socket
- `-s`: Refresh-Intervall in Sekunden
- `-b`: Batch-Modus
- `-m`: Startmodus (`top`, `qps`, `cmd`, `innodb`, `status`)
- `--prompt`: Passwort interaktiv abfragen
- `--nocolor`: ANSI-Farben deaktivieren

### Konfigurationsreihenfolge

Werte werden in dieser Reihenfolge angewendet:

1. Defaults
2. `~/.mytop`
3. Umgebungsvariablen (`MYTOP_HOST`, `MYTOP_PORT`, `MYTOP_USER`, `MYTOP_DB`, `MYTOP_SOCKET`)
4. CLI-Flags

Spätere Quellen überschreiben frühere.

## Interaktive Bedienung (Shortcuts)

Im laufenden Programm stehen unter anderem diese Tasten zur Verfügung:

- `t`, `m`, `c`, `I`, `S`: Moduswechsel
- `q`: Beenden
- `?`: Hilfe anzeigen
- `H`, `o`, `i`, `p`, `R`: Header / Sortierung / Idle / Pause / Resolve
- `u`, `d`, `h`, `F`: User-/DB-/Host-Filter und Reset
- `k`, `K`, `r`: Thread/User-Aktionen und Status zurücksetzen

## Beispiele

Start im Status-Modus mit 2 Sekunden Intervall:

```bash
./mytop -m status -s 2
```

Verbindung über Socket:

```bash
./mytop -S /var/run/mysqld/mysqld.sock -u root -d mysql
```

## Entwicklung

Tests ausführen:

```bash
go test ./...
```
