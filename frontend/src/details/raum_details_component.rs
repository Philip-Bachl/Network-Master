use serde::Deserialize;
use yew::{Html, HtmlResult, Properties, component, html, suspense::use_future_with};

use crate::{model::Raum, util};

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
}

#[component]
pub fn RaumDetailsComponent(
    RaumDetailsComponentProps { raum }: &RaumDetailsComponentProps,
) -> HtmlResult {
    let dose_details = use_future_with(raum.ra_id, |ra_id| async move {
        util::fetch::<Vec<DoseDetail>>(&format!("/api/details/raum/{}", ra_id))
            .await
            .unwrap_or_default()
    })?;

    //TODO: fix dose ordering
    Ok(html! {
        <div id="dosen">
            for dose_detail in dose_details.iter() {
                {render_dose_detail(dose_detail)}
            }
        </div>
    })
}

fn render_dose_detail(dose_detail: &DoseDetail) -> Html {
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
        </div>
    }
}
