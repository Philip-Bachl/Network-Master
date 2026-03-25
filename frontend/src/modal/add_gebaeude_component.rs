use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, UseStateHandle, component, html, use_node_ref};

use crate::{ModalState, model::Gebaeude, util};

#[derive(PartialEq, Properties)]
pub struct AddGebaeudeComponentProps {
    pub modal_state: UseStateHandle<ModalState>,
    pub gebaeude_deps: UseStateHandle<bool>,
}

#[component]
pub fn AddGebaeudeComponent(
    AddGebaeudeComponentProps {
        modal_state,
        gebaeude_deps,
    }: &AddGebaeudeComponentProps,
) -> Html {
    let gebaeude_name_ref = use_node_ref();
    let gebaeude_kommentar_ref = use_node_ref();

    let gebaeude_name_ref_clone = gebaeude_name_ref.clone();
    let gebaeude_kommentar_ref_clone = gebaeude_kommentar_ref.clone();
    let gebaeude_deps_clone = gebaeude_deps.clone();
    let modal_state_clone = modal_state.clone();
    let on_create_button_click = Callback::from(move |_| {
        let gebaeude_name_ref_clone_clone = gebaeude_name_ref_clone.clone();
        let gebaeude_kommentar_ref_clone_clone = gebaeude_kommentar_ref_clone.clone();
        let gebaeude_deps_clone_clone = gebaeude_deps_clone.clone();
        let modal_state_clone_clone = modal_state_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let Some(gebaeude_name) = gebaeude_name_ref_clone_clone
                .cast::<HtmlInputElement>()
                .map(|i| i.value())
                .filter(|v| !v.is_empty())
            else {
                util::alert("Name Feld ist leer");
                return;
            };
            let gebaeude_kommentar = gebaeude_kommentar_ref_clone_clone
                .cast::<HtmlInputElement>()
                .map(|i| i.value());

            let gebaeude = Gebaeude {
                ge_name: gebaeude_name,
                ge_kommentar: gebaeude_kommentar,
            };
            let Ok(serialized_gebaeude) = serde_json::to_string(&gebaeude) else {
                //SMALL TODO: error handling
                return;
            };

            util::fetch_post_with_body("/api/gebaeude", serialized_gebaeude).await;
            gebaeude_deps_clone_clone.set(!*gebaeude_deps_clone_clone);
            modal_state_clone_clone.set(ModalState::Nothing);
        });
    });

    let modal_state_clone = modal_state.clone();
    let on_cancel_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::Nothing);
    });

    html! {
        <div id="addGebaeude">
            <input
                ref={gebaeude_name_ref}
                placeholder="Name"
                id="gebaeudeNameInput"
                type="text"
            />
            <input
                ref={gebaeude_kommentar_ref}
                placeholder="Optional: Kommentar"
                id="gebaeudeKommentarInput"
                type="text"
            />
            <div id="buttons">
                <input type="button" id="createButton" onclick={on_create_button_click} value="Erstellen"/>
                <input type="button" id="cancelButton" onclick={on_cancel_button_click} value="Abbrechen"/>
            </div>
        </div>
    }
}
