# Release-MVP-Checkliste

## Scope & Qualität
- [ ] Phasen 0–6 in `plans.md` vollständig abgehakt.
- [ ] Spec-Tests `01..10` grün.
- [ ] Snapshot-/Golden-Tests validiert und bei Änderungen bewusst aktualisiert.
- [ ] Performance-Smoke-Test (`perf_checks`) dokumentiert ausgeführt.

## Dokumentation
- [ ] `rust-tui/README.md` aktuell (Nutzung, Migration, Tests).
- [ ] Changelog-/Release-Notes erstellt.
- [ ] Bekannte Einschränkungen transparent dokumentiert.

## Versionierung
- [ ] `Cargo.toml` Version erhöht (SemVer).
- [ ] Release-Commit erstellt.
- [ ] Git-Tag gesetzt (z. B. `v0.2.0-mvp`).

## Release-Artefakte
- [ ] `cargo build --release` erfolgreich.
- [ ] Binary/Artefakte archiviert.
- [ ] Veröffentlichung/Ankündigung vorbereitet.
