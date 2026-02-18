const raumNummer = document.querySelector("#raumNummer");
const raumStockwerk = document.querySelector("#raumStockwerk");

const addRaumButton = document.querySelector("#addRaumButton");

const raeumeContainer = document.querySelector("#raeume");

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

/**
 * @type {Raum[]}
 */
let raeume = [];

fetchRaeume().then(r => raeume = r).then(() => renderRaeume());

addRaumButton.addEventListener("mousedown", () => {
    const raum = {
        ra_id: 0,
        ra_ge_name: "Wiedner Hauptstraße",
        ra_nummer: raumNummer.value,
        ra_stockwerk: parseInt(raumStockwerk.value),
        ra_kommentar: null,
    };

    addRaum(raum);
});

//////////////////////////////////////////////////////////////////

async function fetchRaeume() {
    return await (await fetch("/api/raum")).json();
}

function renderRaeume() {
    raeumeContainer.innerHTML = "";
    raeume.forEach(raum => {
        //if (!raum.ra_ge_name.endsWith("0")) return;
        const raumElement = document.createElement("div");
        const raumText = document.createElement("span");
        const deleteButton = document.createElement("span");

        raumText.innerText = generateFullRaumNummer(raum);
        deleteButton.innerText = "X";
        deleteButton.dataset.id = raum.ra_id;

        deleteButton.addEventListener("mousedown", e => {
            const raumId = e.target.dataset.id;
            deleteRaum(raumId);
        });

        raumElement.appendChild(raumText);
        raumElement.appendChild(deleteButton);

        raeumeContainer.appendChild(raumElement);
    });
}

//////////////////////////////////////////////////////////////////


/**
 * 
 * @param {Raum} raum 
 */
async function addRaum(raum) {
    const errorMsg = await (await fetch("/api/raum", { method: "POST", body: JSON.stringify(raum), headers: { "Content-Type": "application/json" } })).text();
    if (errorMsg.length > 0) alert(errorMsg);

    raeume = await fetchRaeume();
    renderRaeume();
}

/**
 * 
 * @param {string} raumIdValue
 */
async function deleteRaum(raumIdValue) {
    const deleteRaum = {
        ra_id: parseInt(raumIdValue),
    };

    const errorMsg = await (await fetch("/api/raum", { method: "DELETE", body: JSON.stringify(deleteRaum), headers: { "Content-Type": "application/json" } })).text();
    if (errorMsg.length > 0) alert(errorMsg);

    raeume = await fetchRaeume();
    renderRaeume();
}

//fetch("/api/gebaeude", {method: "DELETE", body: JSON.stringify({ge_name: "hello"})}).then(r => r.text()).then(d => console.log(d))