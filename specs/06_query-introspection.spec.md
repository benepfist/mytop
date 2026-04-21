# Spec: Query-Inspektion

## Zweck
Ermöglicht Detailanalyse laufender Queries.

## Funktionalitäten
- `FullQueryInfo(id)`: zeigt komplette SQL der Thread-ID aus Cache.
- `Explain(id)`: führt `USE <db>` und anschließend `EXPLAIN <sql>` aus.
- `PrintTable(rows)`: formatiert EXPLAIN-Ausgabe in lesbarer Schlüssel/Wert-Form.

## Voraussetzungen
- Ziel-Thread muss zuvor in `GetData` gesehen worden sein (Cache befüllt).

## Fehlerfälle
- Unbekannte Thread-ID führt zu `*** Invalid id. ***`.
