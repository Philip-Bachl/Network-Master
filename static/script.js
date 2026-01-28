fetch("/api/gebaeude").then(r => r.json()).then(d => console.log(d));

//fetch("/api/gebaeude", {method: "DELETE", body: JSON.stringify({ge_name: "hello"})}).then(r => r.text()).then(d => console.log(d))