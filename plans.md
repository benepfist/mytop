# Umsetzungsplan: TUI-Anwendung in Rust (mytop Rewrite)

## Zielbild
Eine funktionale, interaktive Terminal-Anwendung in Rust, die den Bedienfluss von `mytop` (Perl) reproduziert, dabei aber modular, testbar und schrittweise erweiterbar bleibt.

---

## Kurzvergleich: altes Perl-Script vs. Rust-Rewrite

### 1) Architektur
- **Perl (`mytop`)**: monolithisches Script mit globalem Zustand (`%config`, Caches, DB-Handle), direkte Steuerung von CLI, Rendering und DB-Zugriff in einer Datei.
- **Rust (`rust-tui`)**: modulare Aufteilung nach Verantwortlichkeiten (`startup`, `loop_modes`, `top_view`, `commands`, `filters`, `introspection`, `summaries`, `output`, `help`, `utils`).

### 2) Feature-Abdeckung (Ist-Stand)
- **Bereits im Rust-Core modelliert**:
  - Konfiguration + Merge + DSN-Bau
  - Modus-/Loop-Grundlogik
  - Filter- und Sortierverhalten
  - Query-Introspection (Full Query / Explain)
  - Summaries (QPS, Commands, Status)
  - Hilfs- und Utility-Funktionen
- **Noch offen bis zur echten TUI-Parität**:
  - Vollständiges CLI-Parsing im Binary (`main.rs` ist aktuell Platzhalter)
  - Reale DB-Verbindung + Datenabruf (`SHOW FULL PROCESSLIST`, `SHOW STATUS`, `SHOW VARIABLES`, `SHOW ENGINE INNODB STATUS`)
  - Nicht-blockierende Tastatursteuerung inkl. kompletter Shortcut-Matrix
  - Terminal-Rendering inkl. Header-/Thread-Tabelle in Refresh-Zyklen

### 3) Technische Unterschiede / Chancen
- **Perl**: schnelle direkte Umsetzung, aber eng gekoppelte Logik erschwert refactoring.
- **Rust**: höhere Initialkomplexität, dafür bessere Typsicherheit, klarere Testbarkeit und langfristig wartbarere TUI-Implementierung.

---

## Phasenplan mit TODOs

## Phase 0 – Baseline & Paritätskatalog
**Ziel:** verbindliche Referenz schaffen, welche Perl-Funktion in welches Rust-Modul überführt wird.

- [x] `mytop`-Funktionen in eine Mapping-Tabelle überführen (Perl-Subroutine -> Rust-Modul/Funktion).
- [x] Muss-/Kann-Features festlegen (MVP vs. Nice-to-have).
- [x] Akzeptanzkriterien pro Modus definieren (`top`, `qps`, `cmd`, `innodb`, `status`).
- [x] Spec-Dateien (01–10) als Definition-of-Done pro Bereich referenzieren.

**Ergebnisartefakt:** `docs/parity-matrix.md` + Prioritätenliste.

## Phase 1 – CLI/Startup vervollständigen
**Ziel:** Binary startet mit echten Parametern wie das Perl-Original.

- [x] CLI-Argumente in `main.rs` anbinden (host, port, user, pass, db, socket, delay, batch, prompt, color/no-color, mode, sort, filter).
- [x] Konfigurationsdatei-Handling (`~/.mytop`) vollständig integrieren.
- [x] Prioritätsregeln validieren: Defaults < Config-File < CLI.
- [x] DSN-/Connection-Setup gegen mysql-Client-Library implementieren.
- [x] Fehlertexte für Verbindungsfehler kompatibel/formnah zum bisherigen Verhalten gestalten.

**Ergebnisartefakt:** lauffähiger Non-Interactive Startpfad.

## Phase 2 – Datenzugriffsschicht (DB Polling)
**Ziel:** reproduzierbare, testbare Abfragen als Fundament für Rendering und Kommandos.

- [x] DB-Abstraktion (Trait/Interface) für echte DB + Mocking schaffen.
- [x] Queries kapseln:
  - [x] `SHOW FULL PROCESSLIST`
  - [x] `SHOW STATUS`
  - [x] `SHOW VARIABLES`
  - [x] `SHOW ENGINE INNODB STATUS`
- [x] Caches portieren (`qcache`, User/DB-Zuordnungen, Status-Vergleich für Delta-Werte).
- [x] Polling-Takt (Delay) und Zeitmessung robust machen.

**Ergebnisartefakt:** testbare `data`-Schicht mit Mock-Tests.

## Phase 3 – Interaktive Loop & Eingabemodell
**Ziel:** Bedienbarkeit wie im Perl-Tool inkl. Moduswechsel und Kommandos.

- [x] Event-Loop für interaktiv vs. Batch stabilisieren.
- [x] Keybinding-Matrix umsetzen (`t/m/c/I/S/q`, plus Kommandos wie kill/explain/filter/sort).
- [x] Eingabe-Submodi (Prompt-Zeile) für Commands implementieren.
- [x] Validierung und Nutzerfeedback für Fehlbedienung nachziehen (`*** Invalid id. ***` etc.).

**Ergebnisartefakt:** interaktive Steuerung Ende-zu-Ende.

## Phase 4 – TUI-Rendering (MVP)
**Ziel:** fortlaufend aktualisierte Textoberfläche mit klarer Informationshierarchie.

- [x] Header (Uptime, QPS, Slow, Key-Efficiency, Threads) vollständig rendern.
- [x] Prozessliste mit Sortierung, Filtern, Idle-Ausblendung und Host-Normalisierung rendern.
- [x] Ansichten für `qps`, `cmd`, `status`, `innodb` ausgeben.
- [x] Farbunterstützung optional aktivieren/deaktivieren (inkl. Windows/NoColor-Regeln).
- [x] Pager-Integration für längere Ausgaben (`less` fallback `more`).

**Ergebnisartefakt:** nutzbare MVP-TUI.

## Phase 5 – Kommandoparität & Introspection
**Ziel:** tiefe Analysefunktionen (Full Query/Explain/Kill) stabil verfügbar.

- [x] Full Query Info aus Cache + Prozessliste korrekt auflösen.
- [x] `EXPLAIN`-Workflow inkl. ggf. `USE <db>` voranstellen.
- [x] Kill-Kommandos (Thread/User) inklusive Sicherheits-/Bestätigungslogik implementieren.
- [x] Ausgabeformat von Tabellen und Fehlermeldungen vereinheitlichen.

**Ergebnisartefakt:** Funktionsparität der wichtigsten Interaktionskommandos.

## Phase 6 – Stabilisierung, Tests, Release
**Ziel:** robuste Qualität und dokumentierter Rollout.

- [x] Spec-Driven Tests (01–10) als rust-native Test-Suite vollständig abdecken.
- [x] Snapshot-/Golden-Tests für Render-Ausgaben ergänzen.
- [x] Performance-Checks für große Prozesslisten (Sort/Filter/Render-Zeit).
- [x] README + Nutzung + Migrationshinweise von Perl auf Rust finalisieren.
- [x] Versioniertes Release (MVP-Tag) vorbereiten.

**Ergebnisartefakt:** Release-kandidatenfähige Rust-TUI.

## Phase 7 – Paritätsmatrix-Restarbeiten (Teilimplementierungen schließen)
**Ziel:** alle in `docs/parity-matrix.md` als **„Teilweise“** markierten Module bis zur vollständigen Umsetzungsparität fertigstellen.

### Betroffene Module (Audit aus Paritätsmatrix)
- `startup`
- `loop_modes`
- `top_view`
- `commands`
- `filters`
- `introspection`
- `output`
- `summaries`
- `help`

### Aufgabenpakete zur Fertigstellung
- [x] `startup`: CLI-Parsing in `main.rs` vollständig mit `Config`/`merge_config` verdrahten; DSN + echte DB-Verbindungsinitialisierung End-to-End anbinden.
- [x] `loop_modes`: vollständigen Event-/Dispatch-Loop mit realen Key-Events und stabilem Moduswechsel (`top/qps/cmd/innodb/status`) nachziehen.
- [x] `top_view`: Header- und Thread-Tabelle mit Live-Daten vollständig rendern; Sortierung/Filter/Host-Normalisierung im End-to-End-Flow verifizieren.
- [x] `commands`: komplette Command-Matrix ergänzen (inkl. Validierung, Feedback-Texte und noch fehlender Interaktionspfade).
- [x] `filters`: Regex-basierte Filter (statt reinem Contains-Fallback) vollständig implementieren und testen.
- [x] `introspection`: Full-Query-/Explain-Pfade an echte SQL-Ausführung, Caches und Fehlerbehandlung anbinden.
- [x] `output`: TUI-/Tabellenausgabe für alle Modi vollständig harmonisieren; Pager-Verhalten für lange Ausgaben robust absichern.
- [x] `summaries`: QPS-/Command-/Status-Summaries auf echte Polling-Intervalle und vollständige Datenerhebung umstellen.
- [x] `help`: vollständige CLI-Hilfe/Shortcut-Übersicht im Binary final integrieren.
- [x] Cross-Cut: für jedes obige Modul DoD-Checks aus Specs `01`–`10` explizit als Testfälle/Golden-Outputs hinterlegen.

**Ergebnisartefakt:** vollständig geschlossene Teilimplementierungen gemäß Paritätsmatrix + nachweisbare Spec-Parität.

---

## Umsetzungsreihenfolge (empfohlen)
1. **Phase 0–2** (Fundament: Parität + Startup + Daten)
2. **Phase 3–4** (Interaktion + sichtbare TUI)
3. **Phase 5–6** (Kommandotiefe + Stabilisierung)
4. **Phase 7** (gezielte Restarbeiten aus Paritätsmatrix)

---

## Abhakbare Gesamt-Todo-Liste (übergreifend)
- [x] Paritätsmatrix final
- [x] CLI vollständig
- [x] DB-Layer + Mocks
- [x] Interaktive Key-Loop
- [x] Rendering MVP
- [x] Introspection/Kill-Kommandos
- [x] Vollständige Spec-Testabdeckung
- [x] Dokumentation + Release
- [x] Teilimplementierungen aus Paritätsmatrix vollständig schließen
