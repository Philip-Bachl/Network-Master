use yew::{Html, Suspense, component, html, use_state};

use crate::{
    details::details_component::DetailsComponent,
    model::{Raum, Schrank},
    sidebar::sidebar_component::SidebarComponent,
};

mod details;
mod model;
mod sidebar;
mod util;

#[derive(PartialEq, Debug)]
pub enum SidebarSelection {
    Schrank(Schrank),
    Raum(Raum),
    Nothing,
}

//TODO: most components clone data to their children
//      -> change to use heap allocation instead (Rc and alike)

#[component]
fn App() -> Html {
    let sidebar_selection = use_state(|| SidebarSelection::Nothing);
    //TODO: resizing sidebar

    let sidebar_fallback = html! {
        <div id="sidebar">
            <div id="sidebarTitle">{"Locations"}</div>
            <div id="sidebarContent"></div>
        </div>
    };
    let details_fallback = html! {
        <div id="details">
            <div id="detailsTitle"></div>
            <div id="detailsContent"></div>
        </div>
    };

    html! {
        <>
            <Suspense fallback={sidebar_fallback}>
                <SidebarComponent sidebar_selection={sidebar_selection.clone()} />
            </Suspense>
            <Suspense fallback={details_fallback}>
                <DetailsComponent sidebar_selection={sidebar_selection} />
            </Suspense>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
