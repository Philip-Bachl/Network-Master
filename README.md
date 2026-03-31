# Network Master

**Eine Fullstack Netzwerkdatenbankimplementation**

## Inhaltsverzeichnis

### User

- Erstellen
- Bearbeiten
- Löschen
- Dose
- Switch

### Developer
- Techstack
- Backend
  - Endpunkte
  - Entities
- Frontend

# User

### Erstellen

Für Gebäude, Räume und Schränke gibt es unten in der Sidebar einen ```+```-Knopf.<br>
Für Dosen und Switches gibt es eigene Buttons in den jeweiligen Räumen und Schränken.<br>

### Bearbeiten

Dosen und Switchports können durch klicken auf die jeweilige schaltfläche auf der rechten Seite (unten auf mobile) ein Bearbeitungsmenu aufmachen.<br> 
In der Sidebar sowie innerhalb eines Schrankes findet sich <ins>unterstrichener Text</ins>. Dieser kann durch klicken bearbeitet werden.<br>
Das Verbinden von Dose und Switchport wird über das Bearbeitungsmenu auf der Dosenseite gemacht.<br>

### Löschen

Das ```X``` bei den meisten UI-Elementen versucht, dieses zu löschen. Sollte das Element nicht leer (Raum mit Dosen, Gebäude mit Schränken, ...) oder noch mit etwas Anderes verbunden sein (Dose zu Switchport) passiert nichts.<br>

### Dose

Wählt man einen Raum in der Sidebar aus werden alle Dosen in dem Raum aufgelistet.<br>
Von links nach Rechts sieht man dort:

- die Dosennummer
- das Gerät, das auf der Dose hängt
- den Switchport auf dem die Dose verbunden ist
- ein Schloss symbol wenn der Switchport mit dot1x verschlüsselt ist
- das Vlan des Ports
- ein Switch symbol zwecks Trennung
- der Switch name
- die Switch IP-Addresse

Klickt man innerhalb eines Raumes auf eine Dose, geht das Bearbeitungsmenu für diese auf.<br>
Dort sieht man:

- die Dosennummer zum Bearbeiten
- das Gebaeude der Dose
- der Raum der Dose
- der Switch mit dem die Dose verbunden ist (sollte eine Verbindung da sein)
- der Switchport mit dem die Dose verbunden ist (sollte eine Verbindung da sein)
- das Gerät, das auf der Dose hängt
- ein optionales Kommentarfeld

## Switch

Wählt man einen Schrank in der Sidebar aus werden alle Switches in dem Schrank aufgelistet.<br>
Dort sieht man dann zuerst den Namen sowie die IP des Switches und darunter alle Switchports. Diese sind so on oben nach unten aufgebaut:

- der Portname
- ein Symbol welches das Verbundene Gerät anzeigt (ist keines Verbunden ist es ein standart RJ45 Port)
- ein Schloss symbol über dem voherig genanntem Symbol das angibt, ob dot1x verschlüsselung verwendet wird.
- darunter das Vlan des Ports
- die Dosennummer (falls verbunden)

Ist der Switchport mit einer Dose verbunden, ist er umrahmt und gibt unten die Dosennummer an.<br>
<br>

Klickt man auf einen Switchport so öffnet sich das Bearbeitungsmenu. Hier kann man auswählen:

- der Switch zu dem dieser Port gehört (so lassen sich Ports inklusive Verbindung verschieben)
- der Name des Ports
- das Vlan
- ob dot1x Verschlüsselung verwendet wird
- ein optionaler Kommentar

<br>

---

<br>

## Developer

## Techstack

- Backend
  - Rocket (rust)
  - sqlx
- Frontend
  - Yew

### Backend

Das Backend ist in Rocket (rust) implementiert.<br>
Es enthält eine voll implementierte **CRUD** API für alle Entities in der Datenbank.<br>
Zusätzlich ist sie ein eigenständiger Webserver welche die API (auf ```(HOST)/api/..```) und alles im ```dist``` Ordner auf (```(HOST)/..```) hostet (ein File names test.txt im ordner ```./dist``` ist unter ```(HOST)/text.txt``` erreichbar)<br>

Die API wird nicht vollends vom mitgelieferten Frontend benützt und kann benutzt werden, um ein eigenes Frontend zu machen.<br>
<br>

### Endpunkte

Grundsätzlich wird über die HTTP-Method entschieden, welche Operation durchgeführt wird:

- **C**REATE: POST
- **R**EAD: GET
- **U**PDATE: PUT
- **D**ELETE: DELETE

Die Endpunkte folgen diesem Modell: ```(HOST)/api/(entity)/```<br>
Wobei zu bedenken ist, dass Umlaute in ihrer ausgeschriebenen Form verwendet werden:<br> **ä &rarr; ae**

```javascript
//Javascript

/*
[
    {ge_name: "Testgebaeude 0", ge_kommentar: null},
    {ge_name: "Testgebaeude 1", ge_kommentar: "Testkommentar"},
    ...
]
*/
let gebaeude = fetch("HOST/api/gebaeude").await.json().await;
```

<br>

Für Frontendspezifische Anfragen wurden zusättzlich folgende Endpunkte implementiert:

- ```(HOST)/details/switch/[switch_id]```: <br>
    ```sql
    SELECT sp.\*, do.do_id, do.do_nummer, dk.dk_name<br>
    FROM sp_switchport as sp<br>
    LEFT JOIN do_dose as do ON do.do_sp_id = sp.sp_id<br>
    LEFT JOIN dk_device_kind as dk ON do.do_dk_id = dk.dk_id<br>
    WHERE sp.sp_sw_id = $1<br>
    ORDER BY sp.sp_port
    ```
    &rarr; **SwitchportDetail**

- ```(HOST)/details/raum/[ra_id]```: <br>
    ```sql
    SELECT do.*, dk.dk_name, sp.*, sw.sw_name, sw.sw_ip<br>
    FROM ra_raum as ra<br>
    INNER JOIN do_dose as do ON do.do_ra_id = ra.ra_id<br>
    LEFT JOIN dk_device_kind as dk ON do.do_dk_id = dk.dk_id<br>
    LEFT JOIN sp_switchport as sp ON do.do_sp_id = sp.sp_id<br>
    LEFT JOIN sw_switch as sw ON sp.sp_sw_id = sw.sw_id<br>
    WHERE ra.ra_id = $1<br>
    ORDER BY do.do_nummer
    ```
    &rarr; **DoseDetail**

---

### Entities

Grundsätzlich sind Felder nicht **null**.

```typescript
//Typescript

class Gebaeude {
    ge_name: string,
    ge_kommentar: string, //nullable
}

class Raum {
    ra_id: number,
    ra_ge_name: string,
    ra_nummer: string,
    ra_stockwerk: number,
    ra_kommentar: string //nullable,
}

class Schrank {
    sc_id: number,
    sc_ge_name: string,
    sc_nummer: string,
    sc_stockwerk: number,
    sc_kommentar: string //nullable,
}

class DeviceKind {
    dk_id: number,
    dk_name: string,
    dk_kommentar: string //nullable,
}

class Dose {
    do_id: number,
    do_ra_id: number,
    do_nummer: string,
    do_sp_id: number, //nullable
    do_dk_id: number, //nullable
    do_kommentar: string //nullable,
}

class Switch {
    sw_id: number,
    sw_name: string,
    sw_sc_id: number,
    sw_ip: string,
    sw_kommentar: string //nullable,
}

class Switchport {
    sp_id: number,
    sp_sw_id: number,
    sp_port: string,
    sp_vlan: number,
    sp_dot1x: bool,
    sp_kommentar: string //nullable,
}
```

## Frontend

Das Frontend ist eine YEW (rust) Webapp auf Webassembly basis.<br>

## Deployment

build_reset.cmd erstellt (und überschreibt wenn schon vorhanden!!!) einen ```deploy``` Ordner.
Dort ist zum einem der ```dist``` Ordner, welcher das frontend enthält und die ```backend.exe```.
Dieser Ordner representiert alle Dependencies und Daten des Projektes und kann beliebig verschoben werden.<br>

die ```backend.exe``` representiert das backend und den Webserver, der alle dateien in ```dist/..``` auf ```(HOST)/..``` hostet.<br>

Für die backend.exe gibt es 3 flags:

- ```--reset```: führt das backend aus, setzte die Datenbank komplett zurück und erstellt sie neu.
- ```--seed```: führt das backend aus und fügt testdaten in die Datenbank ein
- ```--simple```: _deprecated_

Wird backend.exe ohne flags ausgeführt startes es die Application normal.<br>
Um die Application zu starten benötigt backend.exe ein ```.env``` File welches sich im deploy Ordner befinden muss.
Dort ist ist die **```DATABASE_URL```** zu setzen um den Dateinamen des Sqlite Files zu bestimmen.<br>

```toml
DATABASE_URL="database.db"
```
