use yew::{Html, component, html};

#[component]
fn App() -> Html {
    html! {
        { for (0..20).into_iter() }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
