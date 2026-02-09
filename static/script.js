const raumSelect = document.querySelector("#raumSelect");
const doseNummer = document.querySelector("#doseNummer");

const addDoseButton = document.querySelector("#addDoseButton");

class Raum {
    /**@type {number} */
    ra_id;
    /**@type {string} */
    ra_ge_name;
    /**@type {string} */
    ra_nummer;
    /**@type {number} */
    ra_stockwerk;
    /**@type {string} */
    ra_kommentar;
}

class Dose {
    do_id;
    /**@type {number} */
    do_ra_id;
    /**@type {number} */
    do_nummer;
    /**@type {number} */
    do_dk_id;
    /**@type {string} */
    do_kommentar;
}

fetch("/api/raum").then(r => r.json()).then(d => renderRaeume(d));

addDoseButton.addEventListener("mousedown", () => {
    const raumId = raumSelect.value;
    const dose = {
        do_id: 0,
        do_ra_id: parseInt(raumId),
        do_nummer: doseNummer.value,
        do_dk_id: null, //TODO: change!
        do_kommentar: null //TODO: change!
    };

    addDose(dose);
});

/**
 * 
 * @param {Raum[]} raeume 
 */
function renderRaeume(raeume) {
    raeume.forEach(raum => {
        if (!raum.ra_ge_name.endsWith("0")) return;
        const raumElement = document.createElement("option");

        let raumNummer = `${raum.ra_stockwerk}${Math.floor(raum.ra_nummer / 10)}${raum.ra_nummer % 10}`

        raumElement.innerText = raumNummer;
        raumElement.value = raum.ra_id;

        raumSelect.appendChild(raumElement);
    });
}

/**
 * 
 * @param {Raum} raum 
 */
function addRaum(raum) {
    fetch("/api/raum", { method: "POST", body: JSON.stringify(raum), headers: { "Content-Type": "application/json" } })
        .then(r => r.text())
        .then(s => console.log(s));
}

/**
 * 
 * @param {Dose} dose 
 */
function addDose(dose) {
    fetch("/api/dose", { method: "POST", body: JSON.stringify(dose), headers: { "Content-Type": "application/json" } })
        .then(r => r.text())
        .then(s => console.log(s));
}

//fetch("/api/gebaeude", {method: "DELETE", body: JSON.stringify({ge_name: "hello"})}).then(r => r.text()).then(d => console.log(d))