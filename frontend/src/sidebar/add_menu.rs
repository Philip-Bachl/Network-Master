use yew::{Callback, Html, Properties, component};
use yew::{UseStateHandle, html, use_state};

use crate::ModalState;

#[derive(PartialEq, Properties)]
pub struct AddMenuComponentProps {
    pub modal_state: UseStateHandle<ModalState>,
    pub gebaeude_deps: UseStateHandle<bool>,
    pub raeume_deps: UseStateHandle<bool>,
    pub schraenke_deps: UseStateHandle<bool>,
}

#[component]
pub fn AddMenuComponent(
    AddMenuComponentProps {
        modal_state,
        gebaeude_deps,
        raeume_deps,
        schraenke_deps,
    }: &AddMenuComponentProps,
) -> Html {
    let open = use_state(|| false);

    let open_clone = open.clone();
    let on_menu_button_click = Callback::from(move |_| {
        open_clone.set(!*open_clone);
    });

    let open_state = html! {
        <>
            <div id="addMenuTitle">{ "Hinzufügen" }</div>
            <div id="options">
                <div class="option" onclick={
                    let modal_state_clone = modal_state.clone();
                    let gebaeude_deps_clone = gebaeude_deps.clone();
                    Callback::from(move |_| {modal_state_clone.set(ModalState::AddGebaeude(gebaeude_deps_clone.clone()))})
                }>
                    <img src="assets/svg/gebaeude.svg" />
                    <div>{ "Gebäude" }</div>
                </div>
                <div class="option" onclick={
                    let modal_state_clone = modal_state.clone();
                    let raeume_deps_clone = raeume_deps.clone();
                    Callback::from(move |_| {modal_state_clone.set(ModalState::AddRaum(raeume_deps_clone.clone()))})
                }>
                    <img src="assets/svg/raum.svg" />
                    <div>{ "Raum" }</div>
                </div>
                <div class="option" onclick={
                    let modal_state_clone = modal_state.clone();
                    let schraenke_deps_clone = schraenke_deps.clone();
                    Callback::from(move |_| {modal_state_clone.set(ModalState::AddSchrank(schraenke_deps_clone.clone()))})
                }>
                    <img src="assets/svg/schrank.svg" />
                    <div>{ "Schrank" }</div>
                </div>
            </div>
        </>
    };

    html! {
        <div id="addMenu">
            {
                if *open {
                    open_state
                } else {
                    html! {}
                }
            }
            <img src="assets/svg/plus.svg" id="expandButton" onclick={on_menu_button_click} />
        </div>
    }
}
