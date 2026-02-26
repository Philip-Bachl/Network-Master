use crate::model::{Gebaeude, Schrank};
use implicit_clone::sync::IArray;
use yew::{Html, Properties, component, html};

#[derive(Properties, PartialEq)]
pub(super) struct GebaeudeProps {
    pub gebaeude: Gebaeude,
    pub schraenke: IArray<Schrank>,
}

#[component]
pub(super) fn GebaeudeComponent(
    GebaeudeProps {
        gebaeude,
        schraenke,
    }: &GebaeudeProps,
) -> Html {
    html! {
        <div key={&gebaeude.ge_name[..]}>
                <span>{&gebaeude.ge_name}</span><span>{gebaeude.ge_kommentar.as_ref().map(|s| &s[..]).unwrap_or_default()}</span>
        </div>
    }
}
