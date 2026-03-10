use crate::{
    SidebarSelection,
    sidebar::tab_component::TabComponent,
    util::{self, pretty_stockwerk_number},
};
use yew::{
    AttrValue, Callback, Html, HtmlResult, Properties, UseStateHandle, component, html,
    suspense::use_future,
};

use crate::model::{Raum, Schrank};

#[derive(PartialEq, Properties)]
pub struct SidebarComponentProps {
    pub sidebar_selection: UseStateHandle<SidebarSelection>,
}

#[component]
pub fn SidebarComponent(
    SidebarComponentProps { sidebar_selection }: &SidebarComponentProps,
) -> HtmlResult {
    let schrank_list = use_future(|| async {
        util::fetch::<Vec<Schrank>>("/api/schrank")
            .await
            .unwrap_or_default()
    })?;
    let raum_list = use_future(|| async {
        util::fetch::<Vec<Raum>>("/api/raum")
            .await
            .unwrap_or_default()
    })?;

    let full_vec = util::map_schraenke_raeume(schrank_list.to_vec(), raum_list.to_vec());

    Ok(html! {
        <div id="sidebar">
            <div id="sidebarTitle">{"Locations"}</div>
            <div id="sidebarContent">
                for (ge_name, stockwerk_vec) in full_vec {
                    {render_gebaeude(ge_name, stockwerk_vec, sidebar_selection.clone())}
                }
            </div>
        </div>
    })
}

fn render_gebaeude(
    ge_name: AttrValue,
    stockwerk_vec: Vec<(i32, Vec<Schrank>, Vec<Raum>)>,
    sidebar_selection: UseStateHandle<SidebarSelection>,
) -> Html {
    html! {
        <TabComponent title={ge_name} img_url="assets/svg/gebaeude.svg">
            for (stockwerk, schraenke, raeume) in stockwerk_vec {
                {render_stockwerk(stockwerk, schraenke, raeume, sidebar_selection.clone())}
            }
        </TabComponent>
    }
}

fn render_stockwerk(
    stockwerk: i32,
    schraenke: Vec<Schrank>,
    raeume: Vec<Raum>,
    sidebar_selection: UseStateHandle<SidebarSelection>,
) -> Html {
    let add_divider = !schraenke.is_empty() && !raeume.is_empty();
    html! {
        <TabComponent title={pretty_stockwerk_number(stockwerk)} img_url="assets/svg/stockwerk.svg">
            <div class="schraenke">
                for schrank in schraenke {
                    {render_schrank(schrank, sidebar_selection.clone())}
                }
            </div>
            if add_divider {
                <div class="divider" />
            }
            <div class="raeume">
                for raum in raeume {
                    {render_raum(raum, sidebar_selection.clone())}
                }
            </div>
        </TabComponent>
    }
}

fn render_schrank(schrank: Schrank, sidebar_selection: UseStateHandle<SidebarSelection>) -> Html {
    let schrank_clone = schrank.clone();
    let onclick = Callback::from(move |_| {
        sidebar_selection.set(SidebarSelection::Schrank(schrank_clone.clone()));
    });

    html! {
        <div class="schrank" {onclick}>
            <img src="assets/svg/schrank.svg" />
            <div>
                {&schrank.sc_nummer}
            </div>
        </div>
    }
}

fn render_raum(raum: Raum, sidebar_selection: UseStateHandle<SidebarSelection>) -> Html {
    html! {
        <div class="raum">
            <img src="assets/svg/raum.svg" />
            <div>
                {raum.pretty_raum_number()}
            </div>
        </div>
    }
}
