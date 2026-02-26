use yew::{Html, component, html};

use crate::sidebar::sidebar_component::SidebarComponent;

mod model;
mod sidebar;

#[component]
fn App() -> Html {
    html! {
        <SidebarComponent />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
