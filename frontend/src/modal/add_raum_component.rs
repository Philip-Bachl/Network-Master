use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{
    Callback, HtmlResult, NodeRef, Properties, UseStateHandle, component, html,
    suspense::use_future, use_node_ref,
};

use crate::{
    ModalState,
    model::{Gebaeude, Raum},
    util,
};

#[derive(PartialEq, Properties)]
pub struct AddRaumComponentProps {
    modal_state: UseStateHandle<ModalState>,
    raeume_deps: UseStateHandle<bool>,
    start_gebaeude: Gebaeude,
}

#[component]
pub fn AddRaumComponent(
    AddRaumComponentProps {
        modal_state,
        raeume_deps,
        start_gebaeude,
    }: &AddRaumComponentProps,
) -> HtmlResult {
    let gebaeude_list = use_future(|| async move {
        util::fetch_get::<Vec<Gebaeude>>("/api/gebaeude")
            .await
            .unwrap_or_default()
    })?;

    let form_data = FormData {
        raum_gebaeude_select: use_node_ref(),
        raum_nummer: use_node_ref(),
        raum_stockwerk: use_node_ref(),
        raum_kommentar: use_node_ref(),
    };

    let form_data_clone = form_data.clone();
    let raeume_deps_clone = raeume_deps.clone();
    let modal_state_clone = modal_state.clone(); //TODO: shadowing, details below VVV
    let on_create_button_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(handle_create_button_click(
            form_data_clone.clone(),
            raeume_deps_clone.clone(),
            modal_state_clone.clone(),
        ));
    });

    let modal_state_clone = modal_state.clone(); //TODO: unintentionally shadowing (currently does not break anything, could change)
    let on_cancel_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::Nothing);
    });

    //TODO: delete raum

    Ok(html! {
        <div id="addSwitch">
            <select id="gebaeudeSelect" ref={form_data.raum_gebaeude_select}>
                for gebaeude in gebaeude_list.iter().cloned() {
                    <option selected={ start_gebaeude.ge_name == gebaeude.ge_name } value={gebaeude.ge_name.clone()}>{gebaeude.ge_name}</option>
                }
            </select>
            <input
                ref={form_data.raum_nummer}
                placeholder="Raum Nummer"
                id="raumNummerInput"
                type="text"
            />
            <input
                ref={form_data.raum_stockwerk}
                placeholder="Stockwerk: -1 = 1UG, 0 = EG, 1 = 1OG, ..."
                id="raumStockwerkInput"
                type="text"
                pattern="[0-9]+"
            />
            <input
                ref={form_data.raum_kommentar}
                placeholder="Optional: Kommentar"
                id="raumKommentarInput"
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
    raum_gebaeude_select: NodeRef,
    raum_nummer: NodeRef,
    raum_stockwerk: NodeRef,
    raum_kommentar: NodeRef,
}
async fn handle_create_button_click(
    form_data: FormData,
    raeume_deps: UseStateHandle<bool>,
    modal_state: UseStateHandle<ModalState>,
) {
    let Some(raum_gebaeude_name) = form_data
        .raum_gebaeude_select
        .cast::<HtmlSelectElement>()
        .map(|s| s.value())
    else {
        //TODO: error handling
        return;
    };

    let Some(raum_nummer) = form_data
        .raum_nummer
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|v| !v.is_empty())
    else {
        util::alert("Raum Nummer Feld ist leer"); //TODO: consistant naming of messages
        return;
    };

    let Some(raum_stockwerk) = form_data
        .raum_stockwerk
        .cast::<HtmlInputElement>()
        .filter(|i| i.check_validity())
        .and_then(|i| i.value().parse::<i32>().ok())
    else {
        util::alert("Raum Stockwerk Feld ist nicht in der Form: <Zahl>");
        return;
    };

    let raum_kommentar = form_data
        .raum_kommentar
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|rk| !rk.is_empty());

    let raum = Raum {
        ra_id: 0,
        ra_ge_name: raum_gebaeude_name,
        ra_nummer: raum_nummer,
        ra_stockwerk: raum_stockwerk,
        ra_kommentar: raum_kommentar,
    };
    let Ok(serialized_raum) = serde_json::to_string(&raum) else {
        //TODO: error handling
        return;
    };

    util::fetch_post_with_body("api/raum", serialized_raum).await;

    raeume_deps.set(!*raeume_deps);
    modal_state.set(ModalState::Nothing);
}
