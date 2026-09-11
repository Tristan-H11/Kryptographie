# Kryptographie - Akademisches Lernprojekt

Dieses Projekt ist eine Implementierung des RSA-Algorithmus (Verschlüsselung und Signatur) und einiger ElGamal-Derivate auf Basis endlicher Gruppen und im speziellen auch elliptischen Kurven in Rust und TypeScript.

## Installation

Um dieses Projekt lokal zu installieren, führen Sie die folgenden Schritte aus:

1. Klonen Sie das Repository:
```
https://github.com/Tristan-H11/Kryptographie.git
```
2. Wechseln Sie in das Verzeichnis des Projekts (falls nicht bereits geschehen):
```
cd Kryptographie bzw. cd GUI
```
3. Installieren Sie die Abhängigkeiten:
```
npm install
cargo build
```

Falls die Toolchain für Rust nicht installiert ist, kann sie [hier](https://www.rust-lang.org/tools/install) 
heruntergeladen werden.
Falls die Version nicht aktuell ist und für hinterlegte Abhängigkeiten aktualisiert werden muss, öffnen Sie 
CMD (Komandozeile) und geben folgendes ein " rustup update stable ".


## Verwendung

Um das Projekt auszuführen, verwenden Sie die folgenden Befehle:

Für Rust:
```
cargo run --release
```

Für Angular:
```
cd ./GUI
ng serve
```
