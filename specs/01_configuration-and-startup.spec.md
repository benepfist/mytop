# Spec: Konfiguration & Startup

## Zweck
`mytop` initialisiert Laufzeitumgebung, lädt Konfiguration und baut eine DB-Verbindung auf.

## Inputs
- Default-Konfiguration aus `%config`.
- Optional: `~/.mytop` mit `key=value` Paaren.
- CLI-Optionen via `Getopt::Long`.

## Verhalten
- Lädt Defaults und überschreibt diese zuerst aus `~/.mytop`, dann über CLI.
- Unterstützt Host mit `host:port`-Notation.
- Schaltet interaktive Tastatureingaben nur außerhalb von `batchmode` ein.
- Deaktiviert Farbe automatisch auf Windows oder via `--nocolor`.
- Baut DSN auf; Socket hat Vorrang gegenüber Host/Port.
- Optionaler Passwort-Prompt (`--prompt`).
- Bricht mit ausführlicher Fehlermeldung ab, wenn DB-Verbindung fehlschlägt.

## Ergebnis
- Gültiger DB-Handle (`$dbh`) und betriebsbereite Runtime für den Hauptloop.
