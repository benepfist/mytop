# Spec: Filterlogik (User/DB/Host)

## Zweck
Begrenzt sichtbare Threads über flexible Pattern.

## Funktion
`StringOrRegex(input)`

## Regeln
- `/.../` wird als regulärer Ausdruck interpretiert.
- Leere Eingabe setzt Filter auf Match-All (`qr/.*/`).
- Plaintext wird als exakter Regex (`^text$`) umgesetzt.
- Filter werden in der Thread-Ausgabe auf `User`, `db` und `Host` angewendet.

## Reset
- `F` setzt alle drei Filter auf Match-All zurück.
