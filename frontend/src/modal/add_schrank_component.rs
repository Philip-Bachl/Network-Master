use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{
    Callback, HtmlResult, NodeRef, Properties, UseStateHandle, component, html,
    suspense::use_future, use_node_ref,
};

use crate::{
    ModalState,
    model::{Gebaeude, Schrank},
    util,
};

#[derive(PartialEq, Properties)]
pub struct AddSchrankComponentProps {
    pub modal_state: UseStateHandle<ModalState>,
    pub schraenke_deps: UseStateHandle<bool>,
    pub start_gebaeude: Option<Gebaeude>,
}

#[component]
pub fn AddSchrankComponent(
    AddSchrankComponentProps {
        modal_state,
        schraenke_deps,
        start_gebaeude,
    }: &AddSchrankComponentProps,
) -> HtmlResult {
    let gebaeude_list = use_future(|| async move {
        util::fetch_get::<Vec<Gebaeude>>("/api/gebaeude")
            .await
            .unwrap_or_default()
    })?;

    let start_gebaeude_name = start_gebaeude
        .as_ref()
        .or(gebaeude_list.first())
        .map(|sg| &sg.ge_name[..])
        .unwrap_or_default();

    let form_data = FormData {
        schrank_gebaeude_name: use_node_ref(),
        schrank_nummer: use_node_ref(),
        schrank_stockwerk: use_node_ref(),
        schrank_kommentar: use_node_ref(),
    };

    let form_data_clone = form_data.clone();
    let schraenke_deps_clone = schraenke_deps.clone();
    let modal_state_clone = modal_state.clone(); //TINY TODO: shadowing, details below VVV
    let on_create_button_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(handle_create_button_click(
            form_data_clone.clone(),
            schraenke_deps_clone.clone(),
            modal_state_clone.clone(),
        ));
    });

    let modal_state_clone = modal_state.clone(); //TINY TODO: unintentionally shadowing (currently does not break anything, could change)
    let on_cancel_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::Nothing);
    });

    Ok(html! {
        <div id="addSwitch">
            <select id="gebaeudeSelect" ref={form_data.schrank_gebaeude_name}>
                for gebaeude in gebaeude_list.iter().cloned() {
                    <option selected={ start_gebaeude_name == gebaeude.ge_name } value={gebaeude.ge_name.clone()}>{gebaeude.ge_name}</option>
                }
            </select>
            <input
                ref={form_data.schrank_nummer}
                placeholder="Schrank Nummer"
                id="schrankNummerInput"
                type="text"
            />
            <input
                ref={form_data.schrank_stockwerk}
                placeholder="-1=1UG,0=EG,1=1OG, ..."
                id="schrankStockwerkInput"
                type="text"
                pattern="-?[0-9]+"
            />
            <input
                ref={form_data.schrank_kommentar}
                placeholder="Optional: Kommentar"
                id="schrankKommentarInput"
                type="text"
            />
            <div id="buttons">
                <input type="button" id="createButton" onclick={on_create_button_click} value="Erstellen"/>
                <input type="button" id="cancelButton" onclick={on_cancel_button_click} value="Abbrechen"/>
            </div>
        </div>
    })
}

#[derive(Clone)]
struct FormData {
    schrank_gebaeude_name: NodeRef,
    schrank_nummer: NodeRef,
    schrank_stockwerk: NodeRef,
    schrank_kommentar: NodeRef,
}
async fn handle_create_button_click(
    form_data: FormData,
    raeume_deps: UseStateHandle<bool>,
    modal_state: UseStateHandle<ModalState>,
) {
    let Some(schrank_gebaeude_name) = form_data
        .schrank_gebaeude_name
        .cast::<HtmlSelectElement>()
        .map(|s| s.value())
    else {
        //SMALL TODO: error handling
        return;
    };

    let Some(schrank_nummer) = form_data
        .schrank_nummer
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|v| !v.is_empty())
    else {
        util::alert("Raum Nummer Feld ist leer"); //SMALL TODO: consistant naming of messages (should already be the case, just check)
        return;
    };

    let Some(schrank_stockwerk) = form_data
        .schrank_stockwerk
        .cast::<HtmlInputElement>()
        .filter(|i| i.check_validity())
        .and_then(|i| i.value().parse::<i32>().ok())
    else {
        util::alert("Raum Stockwerk Feld ist nicht in der Form: <Zahl>");
        return;
    };

    let schrank_kommentar = form_data
        .schrank_kommentar
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|rk| !rk.is_empty());

    let schrank = Schrank {
        sc_id: 0,
        sc_ge_name: schrank_gebaeude_name,
        sc_nummer: schrank_nummer,
        sc_stockwerk: schrank_stockwerk,
        sc_kommentar: schrank_kommentar,
    };
    let Ok(serialized_schrank) = serde_json::to_string(&schrank) else {
        //SMALL TODO: error handling
        return;
    };

    util::fetch_post_with_body("api/schrank", serialized_schrank).await;

    raeume_deps.set(!*raeume_deps);
    modal_state.set(ModalState::Nothing);
}
