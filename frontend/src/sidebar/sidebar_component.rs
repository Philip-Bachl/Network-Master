use crate::{
    ModalState, SidebarSelection,
    model::Gebaeude,
    sidebar::{add_menu::AddMenuComponent, tab_component::TabComponent},
    util::{self, pretty_stockwerk_number},
};
use yew::{
    AttrValue, Callback, Html, HtmlResult, Properties, UseStateHandle, component, html,
    suspense::use_future_with, use_state,
};

use crate::model::{Raum, Schrank};

#[derive(PartialEq, Properties)]
pub struct SidebarComponentProps {
    pub sidebar_selection: UseStateHandle<SidebarSelection>,
    pub modal_state: UseStateHandle<ModalState>,
}

#[component]
pub fn SidebarComponent(
    SidebarComponentProps {
        sidebar_selection,
        modal_state,
    }: &SidebarComponentProps,
) -> HtmlResult {
    let gebaeude_deps = use_state(|| false);
    let gebaeude_list = use_future_with(*gebaeude_deps, |_| async {
        util::fetch_get::<Vec<Gebaeude>>("/api/gebaeude")
            .await
            .unwrap_or_default()
    })?;

    let schraenke_deps = use_state(|| false);
    let schrank_list = use_future_with(*schraenke_deps, |_| async {
        util::fetch_get::<Vec<Schrank>>("/api/schrank")
            .await
            .unwrap_or_default()
    })?;

    let raeume_deps = use_state(|| false);
    let raum_list = use_future_with(*raeume_deps, |_| async {
        util::fetch_get::<Vec<Raum>>("/api/raum")
            .await
            .unwrap_or_default()
    })?;

    let full_vec = util::map_schraenke_raeume(
        gebaeude_list.to_vec(),
        schrank_list.to_vec(),
        raum_list.to_vec(),
    );

    Ok(html! {
        <div id="sidebar">
            <div id="sidebarTitle">{"Locations"}</div>
            <div id="sidebarContent">
                for (ge_name, stockwerk_vec) in full_vec {
                    {render_gebaeude(ge_name, stockwerk_vec, sidebar_selection.clone())}
                }
            </div>
            <AddMenuComponent modal_state={modal_state.clone()} gebaeude_deps={gebaeude_deps} raeume_deps={raeume_deps} schraenke_deps={schraenke_deps} />
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
    html! {
        <TabComponent title={pretty_stockwerk_number(stockwerk)} img_url="assets/svg/stockwerk.svg">
            if !raeume.is_empty() {
                <TabComponent title="Räume" img_url="assets/svg/raum.svg">
                    <div class="raeume">
                        for raum in raeume {
                            {render_raum(raum, sidebar_selection.clone())}
                        }
                    </div>
                </TabComponent>
            }
            if !schraenke.is_empty() {
                <TabComponent title="Schränke" img_url="assets/svg/schrank.svg">
                    <div class="schraenke">
                        for schrank in schraenke {
                            {render_schrank(schrank, sidebar_selection.clone())}
                        }
                    </div>
                </TabComponent>
            }
        </TabComponent>
    }
}

fn render_schrank(schrank: Schrank, sidebar_selection: UseStateHandle<SidebarSelection>) -> Html {
    let schrank_sc_nummer = schrank.sc_nummer.clone();
    let onclick = Callback::from(move |_| {
        sidebar_selection.set(SidebarSelection::Schrank(schrank.clone()));
    });

    html! {
        <div class="schrank" {onclick}>
            <img src="assets/svg/schrank.svg" />
            <div>
                {schrank_sc_nummer}
            </div>
        </div>
    }
}

fn render_raum(raum: Raum, sidebar_selection: UseStateHandle<SidebarSelection>) -> Html {
    let raum_ra_nummer = raum.ra_nummer.clone();
    let onclick = Callback::from(move |_| {
        sidebar_selection.set(SidebarSelection::Raum(raum.clone()));
    });
    html! {
        <div class="raum" {onclick}>
            <img src="assets/svg/raum.svg" />
            <div>
                {raum_ra_nummer}
            </div>
        </div>
    }
}
