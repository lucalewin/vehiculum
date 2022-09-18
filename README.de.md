## Probleme

1. Das Auto hat keine Rückfahrkamera
2. Sobald das Auto ausgeschaltet wird, wird die Bluetooth-Verbindung getrennt. Wird das Auto wieder angeschaltet, wird die Medien-Quelle des Autos auf FM (Radio) geändert (was ziemlich laut ist). Die Quelle muss dann jedes mal manuell wieder auf Bluetooth gesetzt werden
3. Will man Android Auto (oder Apple CarPlay) benutzen, muss das Handy mit einem USB Kabel mit dem Auto verbunden werden. Das ist umständlich und nimmt viel Platz weg. Zumal es kein Fach/Halter für das Handy gibt


## Limitierungen

Es können keine direkten Änderungen an der Head-Unit (das Display) vorgenommen werden, bzw die Head-Unit vollständig ersetzt werden, da sonst kein Zugriff auf die Funktionen/Einstellungen des Autos möglich sind

### Lösung

Dem entsprechend muss es ein Gerät geben, welches durchgängig mit dem Auto via USB verbunden ist.
Dieses Gerät (möglicherweise ein Raspberry PI 4) würde mit dem Auto angehen und auch wieder ausgehen

## Der Plan

Die Kamera:
- Wird hinten am Auto angebracht
- Wird Per Kabel mit dem Raspberry PI 4 verunden
- Eventuell eine Dashcam einbauen (vorne + hinten)

Sensoren:
- Sensor einbauen, um zu wissen, ob der Rückwärtsgang eingelegt ist
- Gucken, ob Daten von den bereits eingebauten Distanz-Sensoren abgegriffen werden können
- Sensor für Lenkwinkel, damit auf dem bild der Kamera Fahrtlinien angezeigt werden können

Der Raspberry PI 4:
- Dient als Schnittstelle und immitiert Android Auto
- Muss zwischen dem USB Port und dem Auto Computer geschaltet werden.
- Geht an, wenn das Auto angeschaltet wird
- Schaltet sich aus, wenn das Auto ausgeschaltet wird
- Braucht eine Radio-Antenne (damit nicht nur übers Internet Musik gehört werden kann)
- kann via Bluetooth mit dem Handy verbunden werden
  - oder via USB Kabel (USB-A --> USB-?)

## Benötigte Hardware

- Kleine Kamera (die hinten am Auto angebracht werden kann)
- möglicherweise ein zweiter Raspberry PI 4 (einer emuliert die Head-Unit, der andere ist die Schnittstelle)
- USB-A --> USB-C (schwarz, 50cm)
- USB-A --> USB-A ()
- Lötgerät (+ Draht)
- USB Kabel (können am besten welche sein, die keine mehr braucht/die weggeworfen werden sollen) zum verbinden der Kamera/des Raspberry PI 4's mit dem Auto
- Kleiner Monitor (mit HDMI Anschluss) als Testdisplay für den "Head-Unit-emulierenden" Raspberry PI 4

⚠️ *weitere Geräte folgen, je weiter das Projekt voran kommt* ⚠️
