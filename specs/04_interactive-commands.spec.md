# Spec: Interaktive Tastaturkommandos

## Zweck
Erlaubt Laufzeitsteuerung, Filterung und Diagnose aus der UI.

## Unterstützte Kommandos
- Navigation/Modi: `t`, `m`, `c`, `I`, `S`, `q`, `?`
- Darstellung: `H` (Header), `o` (Sortierung), `i` (Idle Toggle), `p` (Pause), `R` (DNS Resolve Toggle)
- Filter: `u`, `d`, `h`, `F` (Reset aller Filter)
- Aktionen: `k` (KILL thread id), `K` (KILL alle Threads eines Users), `r` (FLUSH STATUS)
- Analyse: `f` (vollständige Query), `e` (EXPLAIN), `V` (SHOW VARIABLES)
- Taktung/Debug: `s` (Delay setzen), `#` (Debug), `D` (Dump Config)

## Verhalten
- Kommandos mit Eingabe wechseln temporär in blockierenden Read-Mode.
- Ungültige Eingaben (z. B. nicht-numerische Thread-ID) erzeugen User-Feedback.
