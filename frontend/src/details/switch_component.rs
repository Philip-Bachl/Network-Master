use serde::Deserialize;
use yew::{Html, HtmlResult, Properties, component, html, suspense::use_future_with};

use crate::{model::Switch, util};

#[derive(Deserialize)]
struct SwitchportDetail {
    sp_port: String,
    sp_dot1x: bool,
    do_id: Option<i32>,
    do_nummer: Option<String>,
    dk_name: Option<String>,
}

#[derive(PartialEq, Properties)]
pub struct SwitchComponentProps {
    pub switch: Switch,
}

#[component]
pub fn SwitchComponent(SwitchComponentProps { switch }: &SwitchComponentProps) -> HtmlResult {
    let switchport_details = use_future_with(switch.sw_name.clone(), |sw_name| async move {
        util::fetch::<Vec<SwitchportDetail>>(&format!("/api/details/switch/{}", sw_name))
            .await
            .unwrap_or_default()
    })?;

    Ok(html! {
        <div class="switch">
            <div class="switch-title">{format!("{} - {}", &switch.sw_name, &switch.sw_ip)}</div>
            <div class="switch-content">
                for switchport_detail in switchport_details.iter() {
                    {render_switchport(switchport_detail)}
                }
            </div>
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
