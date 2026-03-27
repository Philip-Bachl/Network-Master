use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::{
    Callback, Html, HtmlResult, Properties, TargetCast, UseStateHandle, component, html,
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
    sp_sw_id: i32,
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
            sp_sw_id: value.sp_sw_id,
            sp_port: value.sp_port,
            sp_vlan: value.sp_vlan,
            sp_dot1x: value.sp_dot1x,
            sp_kommentar: value.sp_kommentar,
        }
    }
}

#[derive(Serialize)]
pub struct DeleteSwitch {
    sw_id: i32,
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
        (switch.sw_id, *switchport_details_deps),
        |deps| async move {
            let sw_id = deps.0;
            util::fetch_get::<Vec<SwitchportDetail>>(&format!(
                "/api/details/switch/{}",
                urlencoding::encode(&sw_id.to_string())
            ))
            .await
            .unwrap_or_default()
        },
    )?;

    let sw_id_clone = switch.sw_id;
    let switches_deps_clone = switches_deps.clone();
    let on_delete_switch_button_click = Callback::from(move |event: yew::MouseEvent| {
        event.stop_propagation();

        let Ok(serialized_delete_switch) =
            serde_json::to_string(&DeleteSwitch { sw_id: sw_id_clone })
        else {
            //SMALL TODO: error handling
            return;
        };
        let switches_deps_clone_clone = switches_deps_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            util::fetch_delete_with_body("/api/switch", serialized_delete_switch).await;
            switches_deps_clone_clone.set(!*switches_deps_clone_clone);
        }); // MEDIUM TODO: add error handling to fetch_delete_with_body and then here to notify the user if theres a foreign key falure (ports connected)
    });

    let switch_clone = switch.clone();
    let on_switch_name_submit = Callback::from(move |event: yew::Event| {
        let input = event.target_unchecked_into::<HtmlInputElement>();
        if !input.check_validity() {
            return;
        }
        let sw_name = input.value();

        update_switch(&switch_clone, sw_name, switch_clone.sw_ip.clone());
    });
    let switch_clone = switch.clone();
    let on_switch_ip_submit = Callback::from(move |event: yew::Event| {
        let input = event.target_unchecked_into::<HtmlInputElement>();
        if !input.check_validity() {
            return;
        }
        let sw_ip = input.value();

        update_switch(&switch_clone, switch_clone.sw_name.clone(), sw_ip);
    });

    let resize_callback = Callback::from(|event: yew::InputEvent| {
        let input = event.target_unchecked_into::<HtmlInputElement>();
        input.set_size(input.value().len().saturating_sub(2).max(1) as u32);
    });

    Ok(html! {
        <div class="switch">
            <div class="switch-title">
                //{format!("{} - {}", &switch.sw_name, &switch.sw_ip)}
                <input
                    type="text"
                    onchange={on_switch_name_submit}
                    oninput={resize_callback.clone()}
                    value={switch.sw_name.clone()}
                    size={switch.sw_name.len().saturating_sub(2).max(1).to_string()}
                />
                <span>{ "-"}</span>
                <input
                    type="text"
                    onchange={on_switch_ip_submit}
                    oninput={resize_callback}
                    value={switch.sw_ip.clone()}
                    size={switch.sw_ip.len().saturating_sub(2).max(1).to_string()}
                    pattern="([0-9]?[0-9]?[0-9]\\.){3}([0-9]?[0-9]?[0-9])"
                />
            </div>

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
            switchport.clone(),
            switch.clone(),
            switchport_details_deps.clone(),
        ));
    });

    html! {
        <div class={format!("switchport{}{}", border, dot1x)} {onclick}>
            <div>{&switchport_detail.sp_port}</div>
            <img src={img_src} />
            <div>{switchport_detail.sp_vlan}</div>
            <div>{switchport_detail.do_nummer.as_deref().unwrap_or_default()}</div>
        </div>
    }
}

fn update_switch(switch: &Switch, sw_name: String, sw_ip: String) {
    //SMALL FEATURE TODO: validation

    if sw_name.is_empty() || sw_ip.is_empty() {
        return;
    }

    let update_switch = Switch {
        sw_name,
        sw_ip,
        ..switch.clone()
    };

    let Ok(serialized_update_switch) = serde_json::to_string(&update_switch) else {
        //SMALL TODO: error handling
        return;
    };

    wasm_bindgen_futures::spawn_local(async move {
        util::fetch_put_with_body("/api/switch", serialized_update_switch).await;
    });
}
