# mytop-tui

`mytop-tui` ist ein Rust-basierter Rewrite des klassischen `mytop`-Tools für MySQL/MariaDB.
Das Ziel ist ein modernes, testbares CLI-Programm mit Fokus auf schnelle Übersicht von
Threads, Queries und Serverzustand.

> Status: Phase 0–6 Plan ist dokumentiert; Core-Module, Polling, Interaktion und Rendering-Bausteine sind umgesetzt.

## Warum dieses Projekt?

- Rust statt Perl für bessere Wartbarkeit und klare Modulgrenzen.
- Verhalten wird über Tests und Spezifikationen abgesichert.
- Grundlage für eine zukünftige interaktive TUI mit bekannten `mytop`-Workflows.

## Voraussetzungen

- Rust Toolchain (empfohlen: stabil)
- Cargo
- Zugriff auf einen MySQL/MariaDB-Server für echte Laufzeitdaten

## Installation / Build

Im Verzeichnis `rust-tui`:

```bash
cargo build
```

Für einen optimierten Build:

```bash
cargo build --release
```

## Tests

```bash
cargo test
```

Ergänzend sind Snapshot-/Golden-Tests in `tests/snapshot_render.rs` und ein Performance-Smoke-Test
in `tests/perf_checks.rs` (explizit per `-- --ignored`) enthalten:

```bash
cargo test --test snapshot_render
cargo test --test perf_checks -- --ignored
```

## Einführung: Nutzung des CLI-Tools

```bash
cargo run -- --help
```

Beispiel:

```bash
cargo run -- --host 127.0.0.1 --port 3306 --user root --db test --mode top
```

## Migration vom Perl-`mytop`

- **Config-Priorität unverändert:** Defaults < `~/.mytop` < CLI.
- **Modi/Shortcuts:** `top`, `qps`, `cmd`, `innodb`, `status` sowie Keyflows sind als State-Maschine modelliert.
- **Introspection:** Full-Query und Explain arbeiten mit Cache + Prozessliste.
- **Kill-Sicherheit:** Kill-Planung für Thread/User enthält Confirmations/Sicherheitsregeln.
- **Output:** Tabellen-/Fehlerformatierung ist vereinheitlicht.

## Projektstruktur

```text
rust-tui/
├── src/main.rs          # Binary-Einstiegspunkt
├── src/lib.rs           # Modul-Exports
├── src/startup.rs       # Konfiguration, CLI, DSN, Connect
├── src/data.rs          # DB-Abstraktion, Polling, Caches
├── src/interactive.rs   # Loop/Key/Prompt-Logik
├── src/top_view.rs      # Top-View-Filter/Sort/Header
├── src/output.rs        # Render- und Formatierungsfunktionen
├── src/commands.rs      # Kommando- und Safety-Parsing
├── src/introspection.rs # Full Query / Explain
├── src/summaries.rs     # QPS-/Command-/Status-Summaries
├── src/help.rs          # Hilfe/Shortcut-Texte
└── src/utils.rs         # Hilfsfunktionen
```

## Release-Vorbereitung (MVP)

Siehe `../docs/release-mvp-checklist.md` für die Versionierungs-/Release-Schritte.
