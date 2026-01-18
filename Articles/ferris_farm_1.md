# Ferris Farm: Part 1 - Gewächshaus mit Rust

2025 habe ich mich immer wieder gefragt, welche Sprache ich 2026 wirklich lernen will: Rust oder C++?

Python war meine erste Programmiersprache und ich arbeite inzwischen souverän genug damit um sagen zu können "der Rest kommt mit der Erfahrung". Und obwohl ich Python mag. kam mit der Zeit dieses Gefühl auf, dass da noch mehr geht. Der Wunsch, näher an das System ranzukommen, Dinge bewusster zu verstehen und nicht alles abstrahiert geschenkt zu bekommen. Kurz gesagt: etwas zu lernen, das weh tut, aber dafür sitzt.

An Weihnachten habe ich mich dann für Rust entschieden. Ja, mir ist bewusst, dass der Rust-Arbeitsmarkt in Deutschland, nett gesagt, überschaubar ist. Trotzdem hat mich der Einstieg überzeugt. The Rust Programming Language ist für mich eine der besten technischen Dokus, die ich je gelesen habe. Und ganz ehrlich: Dabei habe ich Konzepte gelernt, die in den zweieinhalb Jahren Berufsschule und Arbeiten mit Business Central einfach nicht aufkamen.

Am Ende des Buchs baut man einen einfachen multithreaded Webserver, der zwar nicht produktionsreif ist, aber gut genug fürs Heimnetzwerk. Genau daraus ist meine Projektidee entstanden.

Ich betreibe zuhause ein kleines Indoor-Gewächshaus mit Sensoren, LED und Abluft. Die Hardware ist gut, die App vom Hersteller nicht. Die UX/UI ist so semi, Mein Lüfter hat keine Schnittstelle zum Controller und es gibt weder vernünftige Datenauswertung noch eine API. Also mach ich das ding jetzt einfach besser (hoffentlich).

> ⚠️ Ja, es ist Cannabis, nein, ich bin kein Dauerkiffer und nein, ich verkaufe nichts.

## Meine Regeln

Mir geht es in diesem Projekt vor allem um eines: Rust zu lernen. Alles andere ist zweitrangig. Die ESP-Programmierung kommt danach, HTML ist Mittel zum Zweck und CSS kommt mir am besten garnicht unter die Augen.

1. Ich nutze so wenige Bibliotheken und Crates wie möglich. Der Fokus liegt auf der Sprache selbst und auf der Standardbibliothek. Ich will verstehen, was Rust tut, nicht nur, wie man Abhängigkeiten zusammenklickt.
2. Ausgangspunkt ist der multithreaded Webserver aus The Rust Programming Language. Kein Framework, kein Boilerplate, sondern etwas, das ich Schritt für Schritt erweitern kann.
3. Rewrites und Refactors sind ausdrücklich erwünscht. In Produktionscode wäre das oft dumm, hier ist es Teil des Lernprozesses. Wenn ich einen besseren Weg finde, wird der auch ausprobiert.
4. Rust hat Priorität. Wenn das bedeutet, dass der Code für die ESPs erstmal irgendwoher kopiert ist, dann ist das so.
5. Keine KI. Ja, wir leben im KI-Zeitalter. Aber ich möchte lernen, wie Rust funktioniert und nicht, wie ich einem Modell möglichst elegant erkläre, was es für mich schreiben soll.

Das Ziel ist kein schnelles Ergebnis, sondern echtes Verständnis. Wenn das Projekt am Ende länger dauert, aber dafür sitzt, dann hat es genau das erreicht, was ich mir davon verspreche.

## Die Komponenten

Wie bei jedem guten Projekt habe ich erstmal knapp über 100€ ausgegeben um ESP's, Kabel, Sensore, Breadboards usw. zu kaufen.

## Jede gute Beziehung beginnt mit einem Match

Für die ESP32-Sensoren habe ich den Code erstmal stumpf aus Tutorials kopiert. Genauso beim multithreaded Rust-Webserver: der kommt, wie gesagt, aus The Rust Programming Language. Ein paar kleine Anpassungen (z. B. den Request-Body lesen) waren schnell gemacht. Und dann kam das erste interessante Problem: *Wie designe ich URL-Routing, ohne am Ende in 1000 Zeilen match zu versinken?*