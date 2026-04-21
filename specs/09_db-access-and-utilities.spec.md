# Spec: DB-Zugriff & Hilfsfunktionen

## DB-Zugriff
- `Execute(sql)`: prepared statement + execute; bei Prepare-Fehlern Abbruch.
- `Hashes(sql)`: materialisiert Resultset als Array von Hashrefs.

## Hilfsfunktionen
- `Clear()`: Bildschirm löschen (Unix `clear`, Windows Newline-Hack).
- `cmd_s()`: Delay setzen (min. 1 Sekunde).
- `cmd_q()`: ReadMode reset + Exit.
- `Sum(@)`: summiert Werte.
- `commify(n)`: Tausendertrennzeichen.
- `make_short(n)`: kompakte Zahlen (`k/M/G/T`) oder lange Form.
- `FindProg(name)`: sucht Binärdatei in Standardpfaden.
