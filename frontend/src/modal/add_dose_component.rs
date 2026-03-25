use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{
    Callback, HtmlResult, NodeRef, Properties, TargetCast, UseStateHandle, component, html,
    suspense::{use_future, use_future_with},
    use_node_ref, use_state_eq,
};

use crate::{
    ModalState,
    model::{Dose, Gebaeude, Raum},
    util,
};

#[derive(PartialEq, Properties)]
pub struct AddDoseComponentProps {
    pub modal_state: UseStateHandle<ModalState>,
    pub start_raum: Raum,
    pub dosen_deps: UseStateHandle<bool>,
}

/*
    pub struct Dose {
        pub do_id: i32,
        pub do_ra_id: i32, DONE
        pub do_nummer: String,
        pub do_sp_id: Option<i32>, DONE
        pub do_dk_id: Option<i32>, DONE
        pub do_kommentar: Option<String>,
    }
*/

#[component]
pub fn AddDoseComponent(
    AddDoseComponentProps {
        modal_state,
        start_raum,
        dosen_deps,
    }: &AddDoseComponentProps,
) -> HtmlResult {
    let gebaeude_list = use_future(|| async move {
        util::fetch_get::<Vec<Gebaeude>>("/api/gebaeude")
            .await
            .unwrap_or_default()
    })?;
    let selected_gebaeude_name = use_state_eq(|| start_raum.ra_ge_name.clone());

    let raum_list = use_future_with(selected_gebaeude_name.clone(), |ge_name| async move {
        util::fetch_get::<Vec<Raum>>(&format!(
            "/api/raum/gebaeude/{}",
            urlencoding::encode(&ge_name),
        ))
        .await
        .unwrap_or_default()
    })?;

    let on_select_gebaeude = Callback::from(move |event: yew::Event| {
        let select: HtmlSelectElement = event.target_unchecked_into();
        selected_gebaeude_name.set(select.value());
    });

    let form_data = FormData {
        dose_raum_select_ref: use_node_ref(),
        dose_nummer_ref: use_node_ref(),
        dose_kommentar_ref: use_node_ref(),
    };

    let form_data_clone = form_data.clone();
    let dosen_deps_clone = dosen_deps.clone();
    let modal_state_clone = modal_state.clone();
    let on_create_button_click = Callback::from(move |_| {
        let form_data_clone_clone = form_data_clone.clone();
        let dosen_deps_clone_clone = dosen_deps_clone.clone();
        let modal_state_clone_clone = modal_state_clone.clone();
        wasm_bindgen_futures::spawn_local({
            handle_create_button_click(
                form_data_clone_clone,
                dosen_deps_clone_clone,
                modal_state_clone_clone,
            )
        });
    });

    let modal_state_clone = modal_state.clone();
    let on_cancel_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::Nothing);
    });

    Ok(html! {
        <div id="addDose">
            <select id="gebaeudeSelect" onchange={on_select_gebaeude}>
                for gebaeude in gebaeude_list.iter().cloned() {
                    <option selected={ start_raum.ra_ge_name == gebaeude.ge_name } value={gebaeude.ge_name.clone()}>{gebaeude.ge_name}</option>
                }
            </select>
            <select id="raumSelect" ref={form_data.dose_raum_select_ref}>
                for raum in raum_list.iter().cloned() {
                    <option selected={ start_raum.ra_id == raum.ra_id } value={raum.ra_id.to_string()}>{raum.ra_nummer}</option>
                }
            </select>
            <input
                type="text"
                id="doseNummerInput"
                placeholder="Dosennummer"
                ref={form_data.dose_nummer_ref}
            />//MEDIUM TODO: add lables for all inputs everywhere
            <input
                type="text"
                id="doseKommentarInput"
                placeholder="Optional: Kommentar" //SMALL TODO: change "Optional: ..." to "... (Optional)"
                ref={form_data.dose_kommentar_ref}
            />

            <div id="buttons"> //MEDIUM TODO: extract into seperate component
                <input type="button" id="CreateButton" onclick={on_create_button_click} value="Erstellen"/>
                <input type="button" id="CancelButton" onclick={on_cancel_button_click} value="Abbrechen"/>
            </div>
        </div>
    })
}

#[derive(Clone)]
struct FormData {
    dose_raum_select_ref: NodeRef,
    dose_nummer_ref: NodeRef,
    dose_kommentar_ref: NodeRef,
}
async fn handle_create_button_click(
    form_data: FormData,
    dosen_deps: UseStateHandle<bool>,
    modal_state: UseStateHandle<ModalState>,
) {
    let Some(raum_id) = form_data
        .dose_raum_select_ref
        .cast::<HtmlSelectElement>()
        .map(|s| s.value())
        .and_then(|v| v.parse::<i32>().ok())
    else {
        //SMALL TODO: error handling
        return;
    };

    let Some(dose_nummer) = form_data
        .dose_nummer_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|v| !v.is_empty())
    else {
        util::alert("Das Dosennummer Feld ist leer");
        return;
    };

    let kommentar = form_data
        .dose_kommentar_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value());

    let dose = Dose {
        do_id: 0,
        do_ra_id: raum_id,
        do_nummer: dose_nummer,
        do_sp_id: None,
        do_dk_id: None,
        do_kommentar: kommentar,
    };
    let Ok(serialized_dose) = serde_json::to_string(&dose) else {
        //SMALL TODO: error handling
        return;
    };

    util::fetch_post_with_body("api/dose", serialized_dose).await;

    dosen_deps.set(!*dosen_deps);
    modal_state.set(ModalState::Nothing);
}

//BIG TODO: add a way to view kommentar everywhere
