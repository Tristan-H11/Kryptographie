[![Build](https://github.com/Tristan-H11/RSA-Implementation/actions/workflows/build.yml/badge.svg)](https://github.com/Tristan-H11/RSA-Implementation/actions/workflows/build.yml)
[![Test](https://github.com/Tristan-H11/RSA-Implementation/actions/workflows/test.yml/badge.svg)](https://github.com/Tristan-H11/RSA-Implementation/actions/workflows/test.yml)
[![Format Checkstyle](https://github.com/Tristan-H11/RSA-Implementation/actions/workflows/format.yml/badge.svg)](https://github.com/Tristan-H11/RSA-Implementation/actions/workflows/format.yml)
[![Test Coverage](https://github.com/Tristan-H11/RSA-Implementation/actions/workflows/coverage.yml/badge.svg)](https://github.com/Tristan-H11/RSA-Implementation/actions/workflows/coverage.yml)
# RSA Implementation in Kryptographie 1
Dieses Projekt entsteht im Integrationsprojekt Kryptographie 1 und hat eine professionelle Implementation von RSA und dem zugehörigen Signaturverfahren zum Ziel.
### Was bereits in der ersten Woche programmiert werden kann:
- Erweiterter Euklidischer Algorithmus
- Schnelle Exponentiation

### Anforderungen
- Für die Erzeugung von ganz großen Primzahlen soll die Finale Version nicht länger als eine Minute dauern.

### Programmierung
Wenn der Checkstyle (Format) fehlschlägt, einfach auf der Konsole `cargo fmt` ausführen.
`cargo fmt --check` zeigt die verstöße an, ohne die `--check`-Flag korrigiert er die Formatierung direkt.

### Benchmarks
#### Schnelle Exponentiation
```rust
let base = &ubig!(5345890).pow(50);
let exponent = &ubig!(561563).pow(50);
let modul = &ubig!(402).pow(453);

fast_exponentiation(base, exponent, modul);
```
Die Zahlen sind über 1000-stellig und die Berechnung inklusive Erstellung der Zahlen dauert auf einem M2 ungefähr 250ms.test
