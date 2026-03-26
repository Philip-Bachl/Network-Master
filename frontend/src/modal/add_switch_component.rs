use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{
    Callback, Event, HtmlResult, NodeRef, Properties, TargetCast, UseStateHandle, component, html,
    suspense::{use_future, use_future_with},
    use_node_ref, use_state_eq,
};

use crate::{
    ModalState,
    model::{Gebaeude, Schrank, Switch},
    util,
};

#[derive(PartialEq, Properties)]
pub struct AddSwitchComponentProps {
    pub modal_state: UseStateHandle<ModalState>,
    pub start_schrank: Schrank,
    pub switches_deps: UseStateHandle<bool>,
}

#[component]
pub fn AddSwitchComponent(
    AddSwitchComponentProps {
        modal_state,
        start_schrank,
        switches_deps,
    }: &AddSwitchComponentProps,
) -> HtmlResult {
    let gebaeude_list = use_future(|| async move {
        util::fetch_get::<Vec<Gebaeude>>("/api/gebaeude")
            .await
            .unwrap_or_default()
    })?;
    let selected_gebaeude_name = use_state_eq(|| start_schrank.sc_ge_name.clone());

    let schrank_list = use_future_with(selected_gebaeude_name.clone(), |sel_geb| async move {
        if sel_geb.is_empty() {
            return vec![];
        };

        util::fetch_get::<Vec<Schrank>>(&format!(
            "/api/schrank/gebaeude/{}",
            urlencoding::encode(sel_geb.as_str())
        ))
        .await
        .unwrap_or_default()
    })?;

    //let selected_gebaeude_name_clone = selected_gebaeude_name.clone();
    let on_select_gebaeude = Callback::from(move |event: Event| {
        let select: HtmlSelectElement = event.target_unchecked_into();
        selected_gebaeude_name.set(select.value());
    });

    let modal_state_clone = modal_state.clone();
    let form_data = FormData {
        switch_name_ref: use_node_ref(),
        switch_schrank_select_ref: use_node_ref(),
        switch_ip_ref: use_node_ref(),
        switch_kommentar_ref: use_node_ref(),

        switchports_prefix_ref: use_node_ref(),
        switchports_count_ref: use_node_ref(),
    };
    let form_data_clone = form_data.clone();
    let switches_deps_clone = switches_deps.clone();
    let on_create_button_click = Callback::from(move |_| {
        let form_data_clone_clone = form_data_clone.clone();
        let switches_deps_clone_clone = switches_deps_clone.clone();
        let modal_state_clone_clone = modal_state_clone.clone();
        wasm_bindgen_futures::spawn_local({
            handle_create_button_click(
                form_data_clone_clone,
                switches_deps_clone_clone,
                modal_state_clone_clone,
            )
        });
    });

    let modal_state_clone = modal_state.clone();
    let on_cancel_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::Nothing);
    });

    //MEDIUM TODO keyed list everywhere
    //BIG TODO smart pointers to reduce cloning (specifically in the case of lists)
    Ok(html! {
        <div id="addSwitch">
            <select id="gebaeudeSelect" onchange={on_select_gebaeude}>
                for gebaeude in gebaeude_list.iter().cloned() {
                    <option selected={ start_schrank.sc_ge_name == gebaeude.ge_name } value={gebaeude.ge_name.clone()}>{gebaeude.ge_name}</option>
                }
            </select>
            <select id="schrankSelect" ref={form_data.switch_schrank_select_ref}>
                for schrank in schrank_list.iter().cloned() {
                    <option selected={ start_schrank.sc_id == schrank.sc_id }  value={schrank.sc_id.to_string()}>{schrank.sc_nummer}</option>
                }
            </select>
            <input
                ref={form_data.switch_name_ref}
                placeholder="Name"
                id="switchNameInput"
                type="text"
            />
            <input
                ref={form_data.switch_ip_ref}
                placeholder="0.0.0.0"
                id="switchIpInput"
                type="text"
                pattern="([0-9]?[0-9]?[0-9]\\.){3}([0-9]?[0-9]?[0-9])"
            />
            <input
                ref={form_data.switchports_prefix_ref}
                placeholder="Port-Prefix"
                id="switchportsPrefixInput"
                type="text"
            />
            <input
                ref={form_data.switchports_count_ref}
                placeholder="Port-Anzahl"
                id="switchportsCountInput"
                type="text"
                pattern="[0-9]+"
            />
            <input
                ref={form_data.switch_kommentar_ref}
                placeholder="Optional: Kommentar"
                id="switchKommentarInput"
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
    switch_name_ref: NodeRef,
    switch_schrank_select_ref: NodeRef,
    switch_ip_ref: NodeRef,
    switch_kommentar_ref: NodeRef,

    switchports_prefix_ref: NodeRef,
    switchports_count_ref: NodeRef,
}

async fn handle_create_button_click(
    form_data: FormData,
    switches_deps: UseStateHandle<bool>,
    modal_state: UseStateHandle<ModalState>,
) {
    let Some(switch_name) = form_data
        .switch_name_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|v| !v.is_empty())
    else {
        util::alert("Switch Name Feld ist leer");
        return;
    };

    let Some(switch_schrank_id) = form_data
        .switch_schrank_select_ref
        .cast::<HtmlSelectElement>()
        .map(|s| s.value())
        .and_then(|v| v.parse::<i32>().ok())
    else {
        //SMALL TODO: error handling
        return;
    };

    let Some(switch_ip) = form_data
        .switch_ip_ref
        .cast::<HtmlInputElement>()
        .filter(|i| i.check_validity())
        .map(|i| i.value())
        .filter(|v| !v.is_empty())
    else {
        util::alert("Ip Feld ist nicht in der Form: <Zahl>.<Zahl>.<Zahl>.<Zahl>");
        return;
    };

    let switch_kommentar = form_data
        .switch_kommentar_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|sk| !sk.is_empty());

    let switch = Switch {
        sw_id: 0,
        sw_name: switch_name,
        sw_sc_id: switch_schrank_id,
        sw_ip: switch_ip,
        sw_kommentar: switch_kommentar,
    };
    let Ok(serialized_switch) = serde_json::to_string(&switch) else {
        //SMALL TODO: error handling
        return;
    };

    let Some(switchports_prefix) = form_data
        .switchports_prefix_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|v| !v.is_empty())
    else {
        util::alert("Port-Prefix Feld ist leer"); //MEDIUM TODO: make it possible for switchports_prefix to be empty 
        return;
    };

    let Some(switchports_count) = form_data
        .switchports_count_ref
        .cast::<HtmlInputElement>()
        .filter(|i| i.check_validity())
        .map(|i| i.value())
        .filter(|p| !p.is_empty())
    else {
        util::alert("Port-Anzahl Feld ist nicht in der Form: <Zahl>");
        return;
    };

    util::fetch_post_with_body(
        &format!(
            "api/switch/port/{}/{}",
            urlencoding::encode(&switchports_prefix),
            urlencoding::encode(&switchports_count)
        ),
        serialized_switch,
    )
    .await;

    switches_deps.set(!*switches_deps);
    modal_state.set(ModalState::Nothing);
}
