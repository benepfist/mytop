# Go-Rewrite Bootstrap

Diese Struktur ist der Startpunkt für den TUI-Rewrite in Go.

## Verzeichnislayout

- `cmd/mytop`: CLI-Entry-Point.
- `internal/app`: Orchestrierung und zentrale Interfaces.
- `internal/config`: Konfigurationsladen (aktuell ENV + Defaults).
- `internal/tui`: Platzhalter für die neue TUI.

## Nächste sinnvolle Schritte

1. CLI-Flags (z. B. via `flag` oder `cobra`) ergänzen.
2. DB-Adapter abstrahieren und erste Queries einbinden.
3. TUI-Framework auswählen (z. B. Bubble Tea) und Status-View aufbauen.
4. Unit-Tests für Config-Loader und App-Orchestrierung hinzufügen.
