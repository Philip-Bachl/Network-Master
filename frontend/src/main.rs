use yew::{Html, Suspense, UseStateHandle, component, html, use_state_eq};

use crate::{
    details::details_component::DetailsComponent,
    modal::modal_component::ModalComponent,
    model::{Dose, Raum, Schrank, Switch, Switchport},
    sidebar::sidebar_component::SidebarComponent,
};

mod details;
mod modal;
mod model;
mod sidebar;
mod util;

#[derive(PartialEq, Debug, Clone)]
pub enum SidebarSelection {
    Schrank(Schrank),
    Raum(Raum),
    Nothing,
}

#[derive(PartialEq)]
pub enum ModalState {
    AddSwitch(Schrank, UseStateHandle<bool>),
    AddDose(Raum, UseStateHandle<bool>),
    EditSwitchport(Switch, Switchport, UseStateHandle<bool>), //TODO: <-- change to be consistant: either switchport/dose as first or second elements VVV
    EditDose(Dose, Raum, Option<Switchport>, UseStateHandle<bool>),
    AddGebaeude(UseStateHandle<bool>),
    AddRaum(UseStateHandle<bool>),
    AddSchrank(UseStateHandle<bool>),
    Nothing,
}

//TODO: most components clone data to their children
//      -> change to use heap allocation instead (Rc and alike)

#[component]
fn App() -> Html {
    let sidebar_selection = use_state_eq(|| SidebarSelection::Nothing);
    let modal_state = use_state_eq(|| ModalState::Nothing);
    //TODO: resizing sidebar

    let sidebar_fallback = html! {
        <div id="sidebar">
            <div id="sidebarTitle">{"Locations"}</div>
            <div id="sidebarContent"></div>
        </div>
    };
    let details_fallback = html! {
        <div id="details">
            <div id="detailsTitle">{"Loading..."}</div>
            <div id="detailsContent"></div>
        </div>
    };
    let modal_fallback = html! {
        <div id="modal">
            <div id="modalLoading">{ "Loading..." }</div>
        </div>
    };

    html! {
        <>
            <Suspense fallback={sidebar_fallback}>
                <SidebarComponent sidebar_selection={sidebar_selection.clone()} modal_state={modal_state.clone()} />
            </Suspense>
            <Suspense fallback={details_fallback}>
                <DetailsComponent sidebar_selection={sidebar_selection.clone()} modal_state={modal_state.clone()} />
            </Suspense>
            <Suspense fallback={modal_fallback}>
                <ModalComponent modal_state={modal_state} />
            </Suspense>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
