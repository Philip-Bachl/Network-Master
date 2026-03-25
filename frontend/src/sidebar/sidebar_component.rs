use crate::{
    ModalState, SidebarSelection,
    model::Gebaeude,
    sidebar::{add_menu::AddMenuComponent, tab_component::TabComponent},
    util::{self, pretty_stockwerk_number},
};
use serde::Serialize;
use yew::{
    AttrValue, Callback, Html, HtmlResult, MouseEvent, Properties, UseStateHandle, component, html,
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
                    {render_gebaeude(ge_name, stockwerk_vec, sidebar_selection.clone(), gebaeude_deps.clone(), schraenke_deps.clone(), raeume_deps.clone())}
                }
            </div>
            <AddMenuComponent modal_state={modal_state.clone()} gebaeude_deps={gebaeude_deps} raeume_deps={raeume_deps} schraenke_deps={schraenke_deps} />
        </div>
    })
}

#[derive(Serialize)]
pub struct DeleteGebaeude {
    ge_name: String,
}

fn render_gebaeude(
    ge_name: AttrValue, //BIG TODO switch almost all references of String to AttrValue
    stockwerk_vec: Vec<(i32, Vec<Schrank>, Vec<Raum>)>,
    sidebar_selection: UseStateHandle<SidebarSelection>,
    gebaeude_deps: UseStateHandle<bool>,
    schraenke_deps: UseStateHandle<bool>,
    raeume_deps: UseStateHandle<bool>,
) -> Html {
    let ge_name_clone = ge_name.clone();
    let delete_callback = Callback::from(move |event: yew::MouseEvent| {
        event.stop_propagation();
        let Ok(serialized_delete_gebaeude) = serde_json::to_string(&DeleteGebaeude {
            ge_name: ge_name_clone.to_string(),
        }) else {
            //SMALL TODO: error handling
            return;
        };
        let gebaeude_deps_clone = gebaeude_deps.clone();
        wasm_bindgen_futures::spawn_local(async move {
            util::fetch_delete_with_body("/api/gebaeude", serialized_delete_gebaeude).await;
            gebaeude_deps_clone.set(!*gebaeude_deps_clone);
        });
    });

    html! {
        <TabComponent title={ge_name} img_url="assets/svg/gebaeude.svg" delete_callback={Some(delete_callback)}>
            for (stockwerk, schraenke, raeume) in stockwerk_vec {
                {render_stockwerk(stockwerk, schraenke, schraenke_deps.clone(), raeume, raeume_deps.clone(), sidebar_selection.clone())}
            }
        </TabComponent>
    }
}

fn render_stockwerk(
    stockwerk: i32,
    schraenke: Vec<Schrank>,
    schraenke_deps: UseStateHandle<bool>,
    raeume: Vec<Raum>,
    raeume_deps: UseStateHandle<bool>,
    sidebar_selection: UseStateHandle<SidebarSelection>,
) -> Html {
    html! {
        <TabComponent title={pretty_stockwerk_number(stockwerk)} img_url="assets/svg/stockwerk.svg" delete_callback={None::<Callback<MouseEvent>>}>
            if !raeume.is_empty() {
                <TabComponent title="Räume" img_url="assets/svg/raum.svg" delete_callback={None::<Callback<MouseEvent>>}>
                    <div class="raeume">
                        for raum in raeume {
                            {render_raum(raum, sidebar_selection.clone(), raeume_deps.clone())}
                        }
                    </div>
                </TabComponent>
            }
            if !schraenke.is_empty() {
                <TabComponent title="Schränke" img_url="assets/svg/schrank.svg" delete_callback={None::<Callback<MouseEvent>>}>
                    <div class="schraenke">
                        for schrank in schraenke {
                            {render_schrank(schrank, sidebar_selection.clone(), schraenke_deps.clone())}
                        }
                    </div>
                </TabComponent>
            }
        </TabComponent>
    }
}

#[derive(Serialize)]
pub struct DeleteSchrank {
    sc_id: i32,
}
fn render_schrank(
    schrank: Schrank,
    sidebar_selection: UseStateHandle<SidebarSelection>,
    schraenke_deps: UseStateHandle<bool>,
) -> Html {
    let sc_id_clone = schrank.sc_id;
    let sidebar_selection_clone = sidebar_selection.clone();

    let schrank_sc_nummer = schrank.sc_nummer.clone();
    let onclick = Callback::from(move |_| {
        sidebar_selection.set(SidebarSelection::Schrank(schrank.clone()));
    });

    let delete_callback = Callback::from(move |event: yew::MouseEvent| {
        event.stop_propagation();

        match *sidebar_selection_clone {
            SidebarSelection::Schrank(Schrank { sc_id, .. }) if sc_id == sc_id_clone => {
                sidebar_selection_clone.set(SidebarSelection::Nothing)
            }
            _ => (),
        }

        let Ok(serialized_delete_schrank) =
            serde_json::to_string(&DeleteSchrank { sc_id: sc_id_clone })
        else {
            //SMALL TODO: error handling
            return;
        };
        let schraenke_deps_clone = schraenke_deps.clone();
        wasm_bindgen_futures::spawn_local(async move {
            util::fetch_delete_with_body("/api/schrank", serialized_delete_schrank).await;
            schraenke_deps_clone.set(!*schraenke_deps_clone);
        });
    });
    html! {
        <div class="schrank" {onclick}>
            <img src="assets/svg/schrank.svg" />
            <div>
                {schrank_sc_nummer}
            </div>
            <img src="assets/svg/plus.svg" class="delete-button" onclick={delete_callback} />
        </div>
    }
}

#[derive(Serialize)]
pub struct DeleteRaum {
    ra_id: i32,
}

fn render_raum(
    raum: Raum,
    sidebar_selection: UseStateHandle<SidebarSelection>,
    raeume_deps: UseStateHandle<bool>,
) -> Html {
    let ra_id_clone = raum.ra_id;
    let sidebar_selection_clone = sidebar_selection.clone();

    let raum_ra_nummer = raum.ra_nummer.clone();
    let onclick = Callback::from(move |_| {
        sidebar_selection.set(SidebarSelection::Raum(raum.clone()));
    });

    let delete_callback = Callback::from(move |event: yew::MouseEvent| {
        event.stop_propagation();

        match *sidebar_selection_clone {
            SidebarSelection::Raum(Raum { ra_id, .. }) if ra_id == ra_id_clone => {
                sidebar_selection_clone.set(SidebarSelection::Nothing)
            }
            _ => (),
        }

        let Ok(serialized_delete_raum) = serde_json::to_string(&DeleteRaum { ra_id: ra_id_clone })
        else {
            //SMALL TODO: error handling
            return;
        };
        let raeume_deps_clone = raeume_deps.clone();
        wasm_bindgen_futures::spawn_local(async move {
            util::fetch_delete_with_body("/api/raum", serialized_delete_raum).await;
            raeume_deps_clone.set(!*raeume_deps_clone);
        });
    });
    html! {
        <div class="raum" {onclick}>
            <img src="assets/svg/raum.svg" />
            <div>
                {raum_ra_nummer}
            </div>
            <img src="assets/svg/plus.svg" class="delete-button" onclick={delete_callback} />
        </div>
    }
}
