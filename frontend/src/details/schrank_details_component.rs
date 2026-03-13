use yew::{HtmlResult, Properties, component, html, suspense::use_future_with};

use crate::{
    details::switch_component::SwitchComponent,
    model::{Schrank, Switch},
    util,
};

#[derive(PartialEq, Properties)]
pub struct SchrankDetailsComponentProps {
    pub schrank: Schrank,
}

#[component]
pub fn SchrankDetailsComponent(
    SchrankDetailsComponentProps { schrank }: &SchrankDetailsComponentProps,
) -> HtmlResult {
    let sc_id = schrank.sc_id;
    let switches = use_future_with(sc_id, |sc_id| async move {
        util::fetch::<Vec<Switch>>(&format!(
            "/api/switch/schrank/{}",
            urlencoding::encode(&sc_id.to_string())
        ))
        .await
        .unwrap_or_default()
    })?;

    Ok(html! {
        <div id="switches">
            for switch in switches.iter().cloned() {
                <SwitchComponent switch={switch} />
            }
        </div>
    })
}
