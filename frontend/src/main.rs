use yew::{Html, Suspense, component, html, use_state};

use crate::{
    details::details_component::DetailsComponent, model::Schrank,
    sidebar::sidebar_component::SidebarComponent,
};

mod details;
mod model;
mod sidebar;
mod util;

#[derive(PartialEq, Debug)]
pub enum SidebarSelection {
    Schrank(Schrank),
    Nothing,
}

#[component]
fn App() -> Html {
    let sidebar_selection = use_state(|| SidebarSelection::Nothing);
    //TODO: resizing sidebar
    html! {
        <>
            <Suspense>
                <SidebarComponent sidebar_selection={sidebar_selection.clone()} />
            </Suspense>
            <Suspense>
                <DetailsComponent sidebar_selection={sidebar_selection} />
            </Suspense>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
