# mytop-tui (Rust rewrite)

Dieses Verzeichnis enthält eine erste **spec-getriebene** Implementierung für den Rewrite von `mytop`.

## Umgesetzte Bereiche

Die vorhandenen Specs aus `../specs` wurden in eigenständige Module überführt:

1. `startup` – Konfig-Merge, Startlogik, DSN-Bildung.
2. `loop_modes` – Moduswechsel und Batch-Verhalten.
3. `top_view` – Headerberechnung, Threadsortierung, Host-Normalisierung.
4. `commands` – Interaktive Eingabevalidierung.
5. `filters` – String/Pattern-Filter inkl. Reset.
6. `introspection` – Full Query + Explain-Ablauf.
7. `summaries` – QPS-, Command- und Status-Zusammenfassungen.
8. `output` – Variablen-/InnoDB-Textausgabe und Pager-Wahl.
9. `utils` – Hilfsfunktionen wie `commify`, `make_short`, `find_prog`.
10. `help` – Shortcut-Hilfe und Dokumentationssektionen.

## TDD-Ansatz

Jedes Modul enthält Unit-Tests, die das erwartete Verhalten aus den Specs absichern.
