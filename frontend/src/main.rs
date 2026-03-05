use yew::{Html, Suspense, component, html};

use crate::sidebar_component::SidebarComponent;

mod model;
mod sidebar_component;
mod tab_component;
mod util;

#[component]
fn App() -> Html {
    //TODO: resizing sidebar
    html! {
        <Suspense>
            <SidebarComponent />
            //<DetailsComponent />
        </Suspense>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
