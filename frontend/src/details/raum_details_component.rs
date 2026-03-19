use serde::{Deserialize, Serialize};
use yew::{
    Callback, Html, HtmlResult, Properties, UseStateHandle, component, html,
    suspense::use_future_with, use_state,
};

use crate::{ModalState, model::Raum, util};

#[derive(Deserialize)]
pub struct DoseDetail {
    do_id: i32,
    do_nummer: String,
    dk_name: Option<String>,
    sp_port: Option<String>,
    sp_dot1x: Option<bool>,
    sp_vlan: Option<i32>,
    sw_name: Option<String>,
    sw_ip: Option<String>,
}

#[derive(PartialEq, Properties)]
pub struct RaumDetailsComponentProps {
    pub raum: Raum,
    pub modal_state: UseStateHandle<ModalState>,
}

#[component]
pub fn RaumDetailsComponent(
    RaumDetailsComponentProps { raum, modal_state }: &RaumDetailsComponentProps,
) -> HtmlResult {
    let dosen_deps = use_state(|| false);
    let dose_details = use_future_with((raum.ra_id, *dosen_deps), |deps| async move {
        let ra_id = deps.0;
        util::fetch_get::<Vec<DoseDetail>>(&format!(
            "/api/details/raum/{}",
            urlencoding::encode(&ra_id.to_string())
        ))
        .await
        .unwrap_or_default()
    })?;

    let modal_state_clone = modal_state.clone();
    let raum_clone = raum.clone();
    let dosen_deps_clone = dosen_deps.clone();
    let on_add_dose_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::AddDose(
            raum_clone.clone(),
            dosen_deps_clone.clone(),
        ));
    });

    //TODO: fix dose ordering
    Ok(html! {
        <div id="dosen">
            for dose_detail in dose_details.iter() {
                {render_dose_detail(dose_detail, dosen_deps.clone())}
            }
            <img src="assets/svg/plus.svg" id="addButton" onclick={on_add_dose_button_click} />
        </div>
    })
}

#[derive(Serialize)]
pub struct DeleteDose {
    do_id: i32,
}
fn render_dose_detail(dose_detail: &DoseDetail, dosen_deps: UseStateHandle<bool>) -> Html {
    let img_src = match dose_detail.dk_name {
        Some(ref dk_name) => format!("assets/svg/{}.svg", dk_name),
        None => String::from("assets/svg/plus.svg"), //TODO: make clickable to add device
    };

    let sw_name = dose_detail.sw_name.as_deref().unwrap_or_default();
    let sw_ip = dose_detail.sw_ip.as_deref().unwrap_or_default();

    let line_classes = if dose_detail.sp_port.is_some() && dose_detail.dk_name.is_some() {
        "line-full"
    } else if dose_detail.sp_port.is_some() || dose_detail.dk_name.is_some() {
        "line-half"
    } else {
        ""
    };

    let do_id = dose_detail.do_id;
    let dosen_deps_clone = dosen_deps.clone();
    let on_delete_dose_button_click = Callback::from(move |_| {
        let Ok(serialized_delete_dose) = serde_json::to_string(&DeleteDose { do_id }) else {
            //TODO: error handling
            return;
        };
        let switches_deps_clone_clone = dosen_deps_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            util::fetch_delete_with_body("/api/dose", serialized_delete_dose).await;
            switches_deps_clone_clone.set(!*switches_deps_clone_clone);
        }); // add error handling to fetch_delete_with_body and then here to notify the user if theres a foreign key falure (ports connected)
    });

    html! {
        <div class="dose">
            <div>{&dose_detail.do_nummer}</div>
            <div class={line_classes}></div>
            <div class="dose-info">
                <img src={img_src} />
                if dose_detail.sp_port.is_some() {
                    <div>{dose_detail.sp_port.as_deref().unwrap_or_default()}</div>
                }
                if dose_detail.sp_dot1x.unwrap_or_default() {
                    <img src="/assets/svg/dot1x.svg" />
                }
                if dose_detail.sp_vlan.is_some() {
                    <div>{dose_detail.sp_vlan.unwrap_or_default()}</div>
                }
                <img src="/assets/svg/switch.svg" />
                <div>
                    {sw_name}
                    if !sw_name.is_empty() && !sw_ip.is_empty() {
                        {" - "}
                    }
                    {sw_ip}
                </div>
            </div>
            <img class="delete-button" src="assets/svg/plus.svg" onclick={on_delete_dose_button_click} />
        </div>
    }
}
