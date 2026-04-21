# Spec: Status- und Command-Summaries

## Zweck
Bietet aggregierte Performance-Sicht ohne Prozesslistenfokus.

## GetQPS
- Liest `SHOW STATUS LIKE "Questions"`.
- Gibt pro Zyklus Differenz zur Vorperiode aus.

## GetCmdSummary
- Liest `SHOW GLOBAL STATUS LIKE 'Com_%'`.
- Normalisiert Kommandonamen (`Com_` entfernen, `_` -> Leerzeichen).
- Berechnet Total, Prozentanteile, Delta und Delta-Prozent je Kommando.

## GetShowStatus
- Liest `SHOW GLOBAL STATUS`.
- Blendet `Com_*` und nichtnumerische Werte aus.
- Zeigt Total + Delta gegenüber letztem Zyklus.
- Optional farbige Hervorhebung bei Änderungen.
