use implicit_clone::sync::IArray;
use yew::{Html, Properties, component, html};

use crate::model::{Schrank, Switch};

#[derive(Properties, PartialEq)]
pub(super) struct SchrankProps {
    schrank: Schrank,
    switches: IArray<Switch>,
}

#[component]
fn SchrankComponent(SchrankProps { schrank, switches }: &SchrankProps) -> Html {
    html! {
        <h1>{ "Schrank" }</h1>
    }
}
