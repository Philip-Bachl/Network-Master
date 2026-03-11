use yew::{Html, Properties, UseStateHandle, component, html};

use crate::{SidebarSelection, details::schrank_details_component::SchrankDetailsComponent};

#[derive(PartialEq, Properties)]
pub struct DetailsComponentProps {
    pub sidebar_selection: UseStateHandle<SidebarSelection>,
}

#[component]
pub fn DetailsComponent(
    DetailsComponentProps { sidebar_selection }: &DetailsComponentProps,
) -> Html {
    let content = match **sidebar_selection {
        SidebarSelection::Schrank(ref schrank) => {
            html! {
                <SchrankDetailsComponent schrank={schrank.clone()}/>
            }
        }
        SidebarSelection::Nothing => html! {},
    };

    html! {
        <div id="details">
            <div id="detailsTitle">{"Details"}</div>
            <div id="detailsContent">
                {content}
            </div>
        </div>
    }
}
