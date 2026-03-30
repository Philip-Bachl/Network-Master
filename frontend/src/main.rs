use yew::{Callback, Html, Suspense, UseStateHandle, component, html, use_state, use_state_eq};

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
    EditSwitchport(Switchport, Switch, UseStateHandle<bool>),
    EditDose(Dose, Raum, Option<Switchport>, UseStateHandle<bool>),
    AddGebaeude(UseStateHandle<bool>),
    AddRaum(UseStateHandle<bool>),
    AddSchrank(UseStateHandle<bool>),
    Nothing,
}

//BIG TODO: most components clone data to their children
//      -> change to use heap allocation instead (Rc and alike)

#[component]
fn App() -> Html {
    let sidebar_selection = use_state_eq(|| SidebarSelection::Nothing);
    let modal_state = use_state_eq(|| ModalState::Nothing);
    //MASSIVE TODO: responsive design (mobile inclusive)

    //FEATURE TODO: resizing sidebar

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

    let sidebar_width = use_state(|| 0);
    let resize = use_state_eq(|| false);

    //SMALL TODO: change from yew::Event / yew::MouseEvent to just Event/MouseEvent
    let resize_clone = resize.clone();
    let sidebar_width_clone = sidebar_width.clone();
    let onmousemove = Callback::from(move |event: yew::MouseEvent| {
        if *resize_clone {
            sidebar_width_clone.set(event.client_x());
        }
    });

    let resize_clone = resize.clone(); //SMALL TODO: use shadowing instead of multiple clones (.._clone_clone) everywhere
    let onmousedown = Callback::from(move |_| {
        resize_clone.set(true);
    });
    let resize_clone = resize.clone();
    let onmouseup = Callback::from(move |_| {
        resize_clone.set(false);
    });

    let no_scroll_class = if *modal_state == ModalState::Nothing {
        ""
    } else {
        "no-scroll"
    };

    html! {
        <main {onmousemove} {onmouseup} class={no_scroll_class}>
            <Suspense fallback={sidebar_fallback}>
                <SidebarComponent sidebar_selection={sidebar_selection.clone()} modal_state={modal_state.clone()} sidebar_width={*sidebar_width} />
            </Suspense>
            <div id="resizeBar" {onmousedown} />
            <Suspense fallback={details_fallback}>
                <DetailsComponent sidebar_selection={sidebar_selection.clone()} modal_state={modal_state.clone()} />
            </Suspense>
            <Suspense fallback={modal_fallback}>
                <ModalComponent modal_state={modal_state} />
            </Suspense>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
