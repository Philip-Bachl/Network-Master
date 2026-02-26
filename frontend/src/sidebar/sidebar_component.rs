use implicit_clone::sync::IArray;
use yew::{Html, UseStateHandle, component, html, use_state_eq};

use crate::{
    model::{Gebaeude, Schrank},
    sidebar::gebaeude_component::GebaeudeComponent,
};

#[component]
pub fn SidebarComponent() -> Html {
    let gebaeude_list = use_state_eq(|| IArray::<Gebaeude>::EMPTY);

    wasm_bindgen_futures::spawn_local(fetch_and_set_gebaeude(gebaeude_list.clone()));

    html! {
        <>
            <link rel="stylesheet" data-trunk="true" href="assets/sidebar.css"/>
            for gebaeude in gebaeude_list.iter() {
                <GebaeudeComponent gebaeude={gebaeude} schraenke={IArray::<Schrank>::EMPTY} />
            }
        </>
    }
}

async fn fetch_and_set_gebaeude(gebaeude_list: UseStateHandle<IArray<Gebaeude>>) {
    let response = match gloo_net::http::Request::get("/api/gebaeude").send().await {
        Ok(res) => res,
        Err(err) => {
            web_sys::window().inspect(|window| {
                let _ = window.alert_with_message(&format!("{err}"));
            });
            return;
        }
    };

    let Ok(gebaeude_json) = response.json::<Vec<Gebaeude>>().await else {
        web_sys::window().inspect(|window| {
            let _ = window.alert_with_message("JSON Parse Error: Response");
        });

        return;
    };

    gebaeude_list.set(gebaeude_json.into());
}
