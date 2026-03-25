const doseNummer = document.querySelector("#doseNummer");
const raumSelect = document.querySelector("#raumSelect");
const deviceKindSelect = document.querySelector("#deviceKindSelect");

const addDoseButton = document.querySelector("#addDoseButton");

const dosenContainer = document.querySelector("#dosen");

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
/**
 * 
 * @param {Raum} raum 
 * @returns {string}
 */
function generateFullRaumNummer(raum) {
    return `${raum.ra_stockwerk}${Math.floor(raum.ra_nummer / 10)}${raum.ra_nummer % 10}`;
}

class DeviceKind {
    /**
     * @type {number}
     */
    dk_id;
    /**
     * @type {string}
     */
    dk_name;
    /**
     * @type {string}
     */
    dk_kommentar;
}

class Dose {
    do_id;
    /**@type {number} */
    do_ra_id;
    /**@type {number} */
    do_nummer;
    /**@type {number} */
    do_sp_id;
    /**@type {number} */
    do_dk_id;
    /**@type {string} */
    do_kommentar;
}
/**
 * 
 * @param {string} fullRaumNummer 
 * @param {Dose} dose
 * @returns {string}
 */
function generateFullDoseNummer(fullRaumNummer, dose) {
    return `${fullRaumNummer}/${dose.do_nummer}`;
}

/**
 * @type {Raum[]}
 */
let raeume = [];

/**
 * @type {DeviceKind[]}
 */
let deviceKinds = [];

/**
 * @type {Dose[]}
 */
let dosen = [];

fetch("/api/raum")
    .then(r => r.json())
    .then(d => {
        raeume = d;
        renderRaeume();
    });

fetch("/api/device_kind")
    .then(r => r.json())
    .then(d => {
        deviceKinds = d;
        renderDeviceKinds();
    });

fetch("/api/dose")
    .then(r => r.json())
    .then(d => {
        dosen = d;
        renderDosen();
    });


addDoseButton.addEventListener("mousedown", () => {
    const raumId = raumSelect.value;

    let deviceKindKey = parseInt(deviceKindSelect.value);
    deviceKindKey = deviceKindKey ? deviceKindKey : null;

    const dose = {
        do_id: 0,
        do_sp_id: null,
        do_ra_id: parseInt(raumId),
        do_nummer: doseNummer.value,
        do_dk_id: deviceKindKey,
        do_kommentar: null
    };

    addDose(dose);
});


raumSelect.addEventListener("change", () => {
    renderDosen();
});

//////////////////////////////////////////////////////////////////

async function fetchDosen() {
    return await (await fetch("/api/dose")).json();
}

function renderRaeume() {
    raeume.forEach(raum => {
        //if (!raum.ra_ge_name.endsWith("0")) return;
        const raumElement = document.createElement("option");

        raumElement.innerText = generateFullRaumNummer(raum);
        raumElement.value = raum.ra_id;

        raumSelect.appendChild(raumElement);
    });
}

function renderDeviceKinds() {
    deviceKinds.forEach(deviceKind => {
        const deviceKindElement = document.createElement("option");

        deviceKindElement.innerText = deviceKind.dk_name;
        deviceKindElement.value = deviceKind.dk_id;

        deviceKindSelect.appendChild(deviceKindElement);
    });
}

function renderDosen() {
    dosenContainer.innerHTML = "";
    dosen
        .filter(dose => dose.do_ra_id == raumSelect.value)
        .forEach(dose => {
            const raum = raeume.filter(r => r.ra_id == dose.do_ra_id)[0];

            const doseElement = document.createElement("div");
            const doseText = document.createElement("span");
            const deleteButton = document.createElement("span");

            doseText.innerText = generateFullDoseNummer(generateFullRaumNummer(raum), dose);

            deleteButton.innerText = "X";
            deleteButton.dataset.id = dose.do_id;

            deleteButton.addEventListener("mousedown", e => {
                const doseId = e.target.dataset.id;
                deleteDose(doseId);
            });

            doseElement.appendChild(doseText);
            doseElement.appendChild(deleteButton);

            dosenContainer.appendChild(doseElement);
        });
}

//////////////////////////////////////////////////////////////////

/**
 * 
 * @param {Dose} dose 
 */
function addDose(dose) {
    fetch("/api/dose", { method: "POST", body: JSON.stringify(dose), headers: { "Content-Type": "application/json" } })
        .then(r => r.text())
        .then(s => console.log(s))
        .then(() => window.location.reload());
}

/**
 * 
 * @param {string} doseIdValue
 */
async function deleteDose(doseIdValue) {
    const deleteDose = {
        do_id: parseInt(doseIdValue),
    };

    const errorMsg = await (await fetch("/api/dose", { method: "DELETE", body: JSON.stringify(deleteDose), headers: { "Content-Type": "application/json" } })).text();
    if (errorMsg.length > 0) alert(errorMsg);

    dosen = await fetchDosen();
    renderDosen();
}

//fetch("/api/gebaeude", {method: "DELETE", body: JSON.stringify({ge_name: "hello"})}).then(r => r.text()).then(d => console.log(d))