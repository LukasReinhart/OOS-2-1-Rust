# Roboter Wettsammeln

Ein Übungsprojekt aus der Fachhochschule Technikum Wien, rekreiert in der [Rust](https://www.rust-lang.org/) Programmiersprache.

## Interesse an Rust

Rust ist eine relativ neue (seit 2015 veröffentlichte) Programmiersprache. Es wird großer Wert auf Compile-Zeit Überprüfungen und sichere Nebenläufigkeit gelegt.

Die FHTW bringt Rust meines Wissens nicht bei, darum lerne ich es in den Sommerferien einfach selbst. Um meinen Fortschritt (und Rust) auf die Probe zu stellen, möchte ich ein umfangreiches Übungsbeispiel aus dem vorherigen Semester nachbauen.

## Ergebnisse

*Noch keine Daten vorhanden, sorry!*

## Spezifikation des Projektes

Dieses Projekt ist eigentlich ein Übungsbeispiel, mit dem **Threads** in **C++** vertieft werden.

> Implementieren Sie ein Programm, dass Threads verwendet, um Roboter darzustellen, die eine Welt nach zufällig verteilten Werten durchsuchen und diese abbauen. Die Welt soll dabei ein zweidimensionales Array beliebiger, nutzerdefinierter Größe (aber zumindest 100 mal 100) sein, das zu Beginn mit zufälligen Werten zwischen 0 und 100 (oder einem nutzerdefinierten Maximum) befüllt wird. Daraufhin soll eine nutzerdefinierte Anzahl an Roboter (aber zumindest 8) zufällig auf die Felder der Welt verteilt werden und dann damit beginnen die Welt zu durchsuchen.

> Dabei sollen folgende Regeln gelten:
> * Roboter können eine von zwei Strategien haben. Eine ist, sich einfach auf ein zufällig gewähltes Nachbarfeld zu bewegen. Die zweite Strategie können Sie sich selbst überlegen.
> * Bewegt sich ein Roboter auf ein Feld, dessen Wert ungleich 0 ist, sammelt er einen Punkt des Feldes ein. Dabei erhöht er seine Punktezahl und verringert den Feldwert dann um 1.
> * Roboter können gleichzeitig auf demselben Feld stehen und dieses abbauen.
> * Danach erfolgt so lange wieder eine Bewegung auf ein Nachbarfeld, bis alle Felder abgebaut sind (also der Feldwert gleich 0). Dann endet die Suchfunktion der Roboter und das Programm gibt die Punktetabelle der Roboter, in absteigender Reihenfolge, aus.

> Folgende Bedingungen sollten erfüllt werden:
> * Verwenden Sie sinnvolle Klassen.
> * Stellen Sie sicher, dass Roboter, die gleichzeitig auf demselben Feld stehen, das Feld korrekt abbauen, es also zu keinen race conditions kommt.
> * Geben Sie nach der Befüllung der Welt mit Werten die Summe der Werte aus und zum Schluss die Summe der Punktezahlen der Roboter, die beiden Summen sollten gleich sein.
> * Schreiben Sie, wo es Sinn macht, Konstruktoren, Destruktoren und Kopierkonstruktoren, beachten Sie dabei mögliche Vererbungen.
> * Verwenden Sie sinnvolle Zugriffsmodifikatoren und dementsprechend Getter und Setter.
> * Teilen Sie Ihren Code sinnvoll in .h- und .cpp-Dateien auf.

Für Rust umgeschrieben:
* Race Conditions werden von Rust bereits beim Kompilieren ausgeschlossen.
* Rust hat keine Header-Dateien. Trotzdem sollte der Code sinnvoll in Module aufgeteilt werden.
