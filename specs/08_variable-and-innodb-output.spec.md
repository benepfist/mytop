# Spec: Variablen- und InnoDB-Ausgabe

## Zweck
Stellt tiefergehende Serverinfos über Pager bereit.

## GetShowVariables
- Liest `SHOW VARIABLES`.
- Formatiert `Variable_name: Value`.
- Pipe-Ausgabe über gefundenen Pager (`less`, fallback `more`).

## GetInnoDBStatus
- Liest `SHOW INNODB STATUS`.
- Gibt `Status`-Text über Pager aus.
- Nutzt denselben Pager-Find-Mechanismus.
