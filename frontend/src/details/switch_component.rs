use serde::{Deserialize, Serialize};
use yew::{
    Callback, Html, HtmlResult, Properties, UseStateHandle, component, html,
    suspense::use_future_with, use_state,
};

use crate::{
    ModalState,
    model::{Switch, Switchport},
    util,
};

#[derive(Deserialize, Clone)]
pub struct SwitchportDetail {
    sp_id: i32,
    sp_sw_name: String,
    sp_port: String,
    sp_vlan: i32,
    sp_dot1x: bool,
    sp_kommentar: Option<String>,

    do_id: Option<i32>,
    do_nummer: Option<String>,
    dk_name: Option<String>,
}

impl From<SwitchportDetail> for Switchport {
    fn from(value: SwitchportDetail) -> Self {
        Switchport {
            sp_id: value.sp_id,
            sp_sw_name: value.sp_sw_name,
            sp_port: value.sp_port,
            sp_vlan: value.sp_vlan,
            sp_dot1x: value.sp_dot1x,
            sp_kommentar: value.sp_kommentar,
        }
    }
}

#[derive(Serialize)]
pub struct DeleteSwitch {
    sw_name: String,
}

#[derive(PartialEq, Properties)]
pub struct SwitchComponentProps {
    pub switch: Switch,
    pub switches_deps: UseStateHandle<bool>,
    pub modal_state: UseStateHandle<ModalState>,
}

#[component]
pub fn SwitchComponent(
    SwitchComponentProps {
        switch,
        switches_deps,
        modal_state,
    }: &SwitchComponentProps,
) -> HtmlResult {
    let switchport_details_deps = use_state(|| false);
    let switchport_details = use_future_with(
        (switch.sw_name.clone(), *switchport_details_deps),
        |deps| async move {
            let sw_name = deps.0.clone();
            util::fetch_get::<Vec<SwitchportDetail>>(&format!(
                "/api/details/switch/{}",
                urlencoding::encode(&sw_name)
            ))
            .await
            .unwrap_or_default()
        },
    )?;

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
                    {render_switchport(switchport_detail, modal_state.clone(), switch.clone(), switchport_details_deps.clone())}
                }
            </div>
            <img class="delete-button" src="assets/svg/plus.svg" onclick={on_delete_switch_button_click} />
        </div>
    })
}

fn render_switchport(
    switchport_detail: &SwitchportDetail,
    modal_state: UseStateHandle<ModalState>,
    switch: Switch,
    switchport_details_deps: UseStateHandle<bool>,
) -> Html {
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

    let switchport: Switchport = switchport_detail.clone().into();
    let onclick = Callback::from(move |_| {
        modal_state.set(ModalState::EditSwitchport(
            switch.clone(),
            switchport.clone(),
            switchport_details_deps.clone(),
        ));
    });

    html! {
        <div class={format!("switchport{}{}", border, dot1x)} {onclick}>
            <img  class={ if switchport_detail.sp_dot1x { "dot1x" } else { "" } } src={img_src} />
            <div>{&switchport_detail.sp_port}</div>
            <div>{switchport_detail.do_nummer.as_deref().unwrap_or_default()}</div>
        </div>
    }
}
