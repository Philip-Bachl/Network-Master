use serde::{Deserialize, Serialize};
use yew::{
    Callback, Html, HtmlResult, Properties, UseStateHandle, component, html,
    suspense::use_future_with,
};

use crate::{model::Switch, util};

#[derive(Deserialize)]
struct SwitchportDetail {
    sp_port: String,
    sp_dot1x: bool,
    do_id: Option<i32>,
    do_nummer: Option<String>,
    dk_name: Option<String>,
}

#[derive(Serialize)]
pub struct DeleteSwitch {
    sw_name: String,
}

#[derive(PartialEq, Properties)]
pub struct SwitchComponentProps {
    pub switch: Switch,
    pub switches_deps: UseStateHandle<bool>,
}

#[component]
pub fn SwitchComponent(
    SwitchComponentProps {
        switch,
        switches_deps,
    }: &SwitchComponentProps,
) -> HtmlResult {
    let switchport_details = use_future_with(switch.sw_name.clone(), |sw_name| async move {
        util::fetch_get::<Vec<SwitchportDetail>>(&format!(
            "/api/details/switch/{}",
            urlencoding::encode(&sw_name)
        ))
        .await
        .unwrap_or_default()
    })?;

    let switch_name_clone = switch.sw_name.clone();
    let switches_deps_clone = switches_deps.clone();
    let on_delete_switch_button_click = Callback::from(move |_| {
        let Ok(serialized_delete_switch) = serde_json::to_string(&DeleteSwitch {
            sw_name: switch_name_clone.clone(),
        }) else {
            //TODO: error handling
            return;
        };
        let switches_deps_clone_clone = switches_deps_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            util::fetch_delete_with_body("/api/switch", serialized_delete_switch).await;
            switches_deps_clone_clone.set(!*switches_deps_clone_clone);
        }); // add error handling to fetch_delete_with_body and then here to notify the user if theres a foreign key falure (ports connected)
    });

    Ok(html! {
        <div class="switch">
            <div class="switch-title">{format!("{} - {}", &switch.sw_name, &switch.sw_ip)}</div>
            <div class="switch-content">
                for switchport_detail in switchport_details.iter() {
                    {render_switchport(switchport_detail)}
                }
            </div>
            <img class="delete-switch-button" src="assets/svg/plus.svg" onclick={on_delete_switch_button_click} />
        </div>
    })
}

fn render_switchport(switchport_detail: &SwitchportDetail) -> Html {
    let img_src = match switchport_detail.dk_name {
        Some(ref dk_name) => format!("assets/svg/{}.svg", dk_name),
        None => String::from("assets/svg/switchport.svg"),
    };

    let border = if switchport_detail.do_id.is_some() {
        " switchport-border"
    } else {
        ""
    };
    let dot1x = if switchport_detail.sp_dot1x {
        " dot1x"
    } else {
        ""
    };

    html! {
        <div class={format!("switchport{}{}", border, dot1x)}>
            <img  class={ if switchport_detail.sp_dot1x { "dot1x" } else { "" } } src={img_src} />
            <div>{&switchport_detail.sp_port}</div>
            <div>{switchport_detail.do_nummer.as_deref().unwrap_or_default()}</div>
        </div>
    }
}
