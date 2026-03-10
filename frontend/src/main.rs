use yew::{Html, Suspense, component, html};

use crate::sidebar::sidebar_component::SidebarComponent;

//mod details_component;
mod model;
mod sidebar;
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
