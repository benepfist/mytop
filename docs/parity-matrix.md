# Paritätsmatrix: Perl `mytop` -> Rust `rust-tui`

## Zweck
Diese Matrix setzt **Phase 0** aus `plans.md` konkret um:
1. Mapping der Perl-Funktionen auf Rust-Module
2. Priorisierung in MVP vs. Nice-to-have
3. Akzeptanzkriterien pro Modus
4. Verknüpfung mit den Spec-Dateien 01–10 als Definition of Done

---

## 1) Mapping-Tabelle (Perl-Subroutine -> Rust-Modul/Funktion)

| Perl-Subroutine / Bereich | Rust-Modul | Rust-Funktion/Typ | Status | Anmerkung |
|---|---|---|---|---|
| Config-Defaults + `~/.mytop` lesen + CLI-Optionen | `startup` | `Config`, `parse_kv_config`, `merge_config` | Teilweise | CLI-Parsing in `main.rs` noch zu verdrahten |
| DSN-Erzeugung / Verbindungsparameter | `startup` | `build_dsn` | Teilweise | DB-Verbindung selbst noch nicht integriert |
| Interaktiv vs. Batch (`Term::ReadKey`) | `startup`, `loop_modes` | `use_interactive_keyboard`, `run_cycles` | Teilweise | echte Key-Events fehlen |
| Moduswechsel (`top/qps/cmd/innodb/status`) | `loop_modes` | `Mode`, `Key`, `switch_mode` | Teilweise | Loop/Dispatch noch rudimentär |
| Header-Kennzahlen (QPS, Slow, Key Eff.) | `top_view` | `compute_header` | Teilweise | Rendering & Live-Daten fehlen |
| Thread-Sortierung / Host-Normalisierung | `top_view` | `sort_threads_by_time`, `normalize_host` | Teilweise | vollständige Tabellenansicht fehlt |
| Kommandos (ID-Validierung, Delay) | `commands` | `parse_thread_id`, `set_delay_secs` | Teilweise | vollständige Command-Matrix fehlt |
| Filter user/db/host + Reset | `filters` | `StringOrRegex`, `Filters::reset` | Teilweise | Regex aktuell als Pattern-Contains modelliert |
| Full Query Info | `introspection` | `full_query_info` | Teilweise | DB-Anbindung/Caches noch auszubauen |
| Explain-Workflow | `introspection` | `explain_sql` | Teilweise | echte SQL-Ausführung fehlt |
| Tabellen-/Textausgabe | `introspection`, `output` | `print_table`, `format_show_variables` | Teilweise | TUI-Renderer fehlt |
| QPS-Delta | `summaries` | `get_qps` | Teilweise | echte Polling-Intervalle fehlen |
| Command-Summary | `summaries` | `command_summary` | Teilweise | Datenerhebung noch offen |
| Status-Summary | `summaries` | `show_status` | Teilweise | Datenerhebung noch offen |
| Hilfe / Shortcut-Übersicht | `help` | `print_help`, `pod_sections` | Teilweise | vollständige CLI-Hilfe in Binary fehlt |
| Hilfsfunktionen (`Clear`, `Sum`, `commify`, `make_short`, `FindProg`) | `utils`, `output` | `clear_command`, `sum`, `commify`, `make_short`, `find_prog`, `find_pager` | Größtenteils | Integration in End-to-End-Flow offen |

---

## 2) Priorisierung: MVP vs. Nice-to-have

## MVP (muss für erste nutzbare Rust-TUI)
- Vollständiges Startup + Config-Merge + CLI-Parameterverdrahtung
- DB-Verbindung + Polling für:
  - `SHOW FULL PROCESSLIST`
  - `SHOW STATUS`
  - `SHOW VARIABLES`
  - `SHOW ENGINE INNODB STATUS`
- Interaktive Loop inkl. Moduswechsel (`t/m/c/I/S/q`)
- Rendering für `top`, `qps`, `cmd`, `status`, `innodb`
- Basis-Kommandos: Delay setzen, Filter setzen/reset, Full Query anzeigen, Explain vorbereiten
- Batchmode (ein Zyklus) und non-interactive nutzbar

## Nice-to-have (nach MVP)
- Erweiterte Kill-Workflows inkl. Confirmations / Safety-Guards
- Fortgeschrittene Farbschemata/Theming
- Snapshot-/Golden-Tests für umfangreiche Render-Zustände
- Performance-Tuning für sehr große Prozesslisten
- Erweiterte Help-/Doku-Parität inkl. historischer POD-Details

---

## 3) Akzeptanzkriterien pro Modus

## Modus `top`
- Header-Metriken werden aus Statuswerten korrekt berechnet.
- Threads werden sortierbar und filterbar angezeigt.
- Hostnamen werden ohne Port dargestellt.
- Idle-Threads können ein-/ausgeblendet werden.

## Modus `qps`
- QPS wird als Delta über zwei Messpunkte korrekt angezeigt.
- Anzeige funktioniert bei `delta_secs <= 0` robust (kein Divide-by-zero).

## Modus `cmd`
- Nur `Com_*`-Werte fließen in Command-Zusammenfassung ein.
- Name-Normalisierung (`Com_select` -> `select`) ist korrekt.
- Delta und Prozentwerte sind nachvollziehbar.

## Modus `innodb`
- Ausgabe von `SHOW ENGINE INNODB STATUS` wird vollständig angezeigt.
- Bei langen Ausgaben wird Pager-Fallback (`less`/`more`) korrekt gewählt.

## Modus `status`
- Nicht-`Com_*`-Statuswerte werden gelistet.
- Delta gegen vorherigen Poll wird korrekt berechnet.

---

## 4) Definition of Done via Specs (01–10)

| Spec | Thema | DoD für Phase 0 |
|---|---|---|
| `01_configuration-and-startup` | Konfiguration & Startup | Mapping + MVP-Priorität + Akzeptanzkriterien erfasst |
| `02_main-loop-and-modes` | Main Loop & Modi | Moduskriterien und Zielverhalten dokumentiert |
| `03_top-view-rendering` | Top-View Rendering | `top`-Kriterien festgelegt |
| `04_interactive-commands` | Interaktive Kommandos | Basis-Command-Priorität festgelegt |
| `05_filters-and-patterns` | Filter/Pattern | Filter-Verhalten in MVP berücksichtigt |
| `06_query-introspection` | Full Query / Explain | Introspection-Ziele und Scope fixiert |
| `07_status-and-command-summaries` | Status/Cmd Summaries | `qps/cmd/status`-Kriterien festgelegt |
| `08_variable-and-innodb-output` | Variables / InnoDB Output | Output-Anforderungen festgelegt |
| `09_db-access-and-utilities` | DB Access + Utilities | DB-Queries + Utility-Abdeckung priorisiert |
| `10_help-and-documentation` | Help/Doku | Doku-/Hilfe-Umfang als MVP/Nice-to-have eingeordnet |

---

## 5) Prioritätenliste (Reihenfolge nach Implementierungsnutzen)

1. Startup/CLI verdrahten und DB-Verbindung herstellen
2. Polling-Datenfluss für Processlist/Status/Variables/InnoDB stabil machen
3. Loop + Key-Handling mit robustem Zustandsmodell umsetzen
4. Rendering aller Modi auf Basis echter Daten komplettieren
5. Kommandotiefe (Explain/Kill/etc.) erweitern und absichern
6. Snapshot-/Performance-/Release-Härtung durchführen
