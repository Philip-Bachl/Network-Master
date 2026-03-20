use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, UseStateHandle, component, html, use_node_ref};

use crate::{ModalState, model::Gebaeude, util};

#[derive(PartialEq, Properties)]
pub struct AddGebaeudeComponentProps {
    modal_state: UseStateHandle<ModalState>,
    gebaeude_deps: UseStateHandle<bool>,
}

//TODO: add trigger for this modal

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
    let modal_state_clone = modal_state.clone(); //TODO: shadowing, details below VVV
    let on_create_button_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local({
            let Some(gebaeude_name) = gebaeude_name_ref_clone
                .cast::<HtmlInputElement>()
                .map(|i| i.value())
            else {
                util::alert("Name Feld ist leer");
                return;
            };
            let gebaeude_kommentar = gebaeude_kommentar_ref_clone
                .cast::<HtmlInputElement>()
                .map(|i| i.value());

            let gebaeude = Gebaeude {
                ge_name: gebaeude_name,
                ge_kommentar: gebaeude_kommentar,
            };
            let Ok(serialized_gebaeude) = serde_json::to_string(&gebaeude) else {
                //TODO: error handling
                return;
            };

            gebaeude_deps_clone.set(!*gebaeude_deps_clone);
            modal_state_clone.set(ModalState::Nothing);
            util::fetch_post_with_body("/api/gebaeude", serialized_gebaeude)
        });
    });

    let modal_state_clone = modal_state.clone(); //TODO: unintentionally shadowing (currently does not break anything, could change)
    let on_cancel_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::Nothing);
    });

    //TODO: delete Gebaeude

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
