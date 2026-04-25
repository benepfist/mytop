# mytop-tui

`mytop-tui` ist ein Rust-basierter Rewrite des klassischen `mytop`-Tools für MySQL/MariaDB.
Das Ziel ist ein modernes, testbares CLI-Programm mit Fokus auf schnelle Übersicht von
Threads, Queries und Serverzustand.

> Status: Der Core ist bereits als Bibliothek mit Spec-orientierten Modulen umgesetzt.
> Der ausführbare CLI-Einstiegspunkt ist aktuell ein Platzhalter und wird schrittweise ausgebaut.

## Warum dieses Projekt?

- Rust statt Perl für bessere Wartbarkeit und klare Modulgrenzen.
- Verhalten wird über Tests und Spezifikationen abgesichert.
- Grundlage für eine zukünftige interaktive TUI mit bekannten `mytop`-Workflows.

## Voraussetzungen

- Rust Toolchain (empfohlen: stabil)
- Cargo
- Optional später: Zugriff auf einen MySQL/MariaDB-Server für echte Laufzeitdaten

## Installation / Build

Im Verzeichnis `rust-tui`:

```bash
cargo build
```

Für einen optimierten Build:

```bash
cargo build --release
```

## Einführung: Nutzung des CLI-Tools

### 1) Anwendung starten

```bash
cargo run
```

Aktuell gibt die Anwendung einen Start-Hinweis aus (`mytop-tui rewrite core loaded`).
Das bestätigt, dass Binary + Runtime korrekt gebaut wurden.

### 2) Kernlogik über Tests verifizieren

Da viele CLI-Funktionen derzeit als Module implementiert sind, ist `cargo test` der wichtigste
Weg, das Verhalten reproduzierbar zu prüfen:

```bash
cargo test
```

Damit werden u. a. folgende Bereiche abgedeckt:

- Konfigurations-Merge und DSN-Erzeugung
- Interaktive Eingabevalidierung
- Filter- und Sortierlogik
- Hilfetexte und Ausgabeformate

### 3) Geplanter CLI-Workflow

Die Modulstruktur bildet bereits den späteren Bedienfluss eines klassischen `mytop` ab:

- **Startup:** Defaults + Config + CLI-Parameter zusammenführen
- **Loop & Modes:** Top-/QPS-/Command-/Status-Sichten wechseln
- **Commands:** Interaktive Befehle, Thread-ID-Handling, Delay-Steuerung
- **Output:** Textausgaben (z. B. Status, InnoDB, Variablen)

## Projektstruktur

```text
rust-tui/
├── src/main.rs          # Binary-Einstiegspunkt
├── src/lib.rs           # Modul-Exports
├── src/startup.rs       # Konfiguration und DSN
├── src/loop_modes.rs    # Loop-/Modus-Logik
├── src/top_view.rs      # Top-Ansicht und Sortierung
├── src/commands.rs      # Interaktive Kommandos
├── src/filters.rs       # Filter-Verhalten
├── src/introspection.rs # Full Query / Explain
├── src/summaries.rs     # QPS-/Command-/Status-Summaries
├── src/output.rs        # Ausgabe-Rendering
├── src/utils.rs         # Hilfsfunktionen
└── src/help.rs          # Hilfe/Shortcut-Texte
```

## Nächste Schritte

- Echte CLI-Argumente (z. B. Host, Port, User, Socket, Batch-Mode) an `main.rs` anbinden
- Datenbankverbindung integrieren
- Interaktive Terminal-Darstellung vervollständigen

---

Historischer Kontext: Dieses Projekt ist ein Rewrite des ursprünglichen `mytop`-Ansatzes,
mit Fokus auf moderne Rust-Entwicklung und testgetriebenes Vorgehen.
