use yew::{Html, Properties, UseStateHandle, component, html};

use crate::{
    ModalState, SidebarSelection,
    details::{
        raum_details_component::RaumDetailsComponent,
        schrank_details_component::SchrankDetailsComponent,
    },
};

#[derive(PartialEq, Properties)]
pub struct DetailsComponentProps {
    pub sidebar_selection: UseStateHandle<SidebarSelection>,
    pub modal_state: UseStateHandle<ModalState>,
}

#[component]
pub fn DetailsComponent(
    DetailsComponentProps {
        sidebar_selection,
        modal_state,
    }: &DetailsComponentProps,
) -> Html {
    let (title, content) = match **sidebar_selection {
        SidebarSelection::Schrank(ref schrank) => (
            format!("{} - {}", schrank.sc_nummer, schrank.sc_ge_name),
            html! {
                <SchrankDetailsComponent schrank={schrank.clone()} modal_state={modal_state.clone()} />
            },
        ),
        SidebarSelection::Raum(ref raum) => (
            format!("{} - {}", raum.ra_nummer, raum.ra_ge_name),
            html! {
                <RaumDetailsComponent raum={raum.clone()} modal_state={modal_state} />
            },
        ),
        SidebarSelection::Nothing => (String::from("Details"), html! {}),
    };

    html! {
        <div id="details">
            <div id="detailsTitle">{title}</div>
            <div id="detailsContent">
                {content}
            </div>
        </div>
    }
}
