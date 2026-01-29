const gebaeudeContainer = document.querySelector("#gebaeude-container");
const gebaeudeTemplate = document.querySelector("#template-gebaeude");

class Gebaeude {
    /**@type {string} */
    ge_name;

    constructor(ge_name) {
        this.ge_name = ge_name;
    }
}

fetch("/api/gebaeude").then(r => r.json()).then(d => renderGebaeude(d));

/**
 * @param {Gebaeude[]} gebaeude 
 */
function renderGebaeude(gebaeude) {
    gebaeude.forEach(element => {
        const newGebaeude = gebaeudeTemplate.cloneNode(true);
        newGebaeude.id = "";
        newGebaeude.children.item(0).innerText = "Name: " + element.ge_name;
        gebaeudeContainer.appendChild(newGebaeude);
    });
}

//fetch("/api/gebaeude", {method: "DELETE", body: JSON.stringify({ge_name: "hello"})}).then(r => r.text()).then(d => console.log(d))