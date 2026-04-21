# Spec: Hauptloop & Modi

## Zweck
Steuert den zyklischen Ablauf und den Wechsel zwischen Betriebsmodi.

## Modi
- `top`: Thread-Übersicht + Header-Metriken.
- `qps`: fortlaufende Queries/Sekunde.
- `cmd`: Kommandostatistik (`Com_*`).
- `innodb`: Ausgabe von `SHOW INNODB STATUS` über Pager.
- `status`: globale Statuszähler mit Delta.

## Verhalten
- Führt je Modus die entsprechende Funktion aus.
- In `batchmode` wird genau ein Zyklus ausgeführt und beendet.
- In interaktivem Betrieb werden Tasteneingaben mit Timeout (`delay`) verarbeitet.
- Umschaltung zwischen Modi via Keys (`t`, `m`, `c`, `I`, `S`, `q`).
