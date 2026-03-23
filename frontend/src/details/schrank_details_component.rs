use yew::{
    Callback, HtmlResult, Properties, UseStateHandle, component, html, suspense::use_future_with,
    use_node_ref, use_state,
};

use crate::{
    ModalState,
    details::switch_component::SwitchComponent,
    model::{Schrank, Switch},
    util,
};

#[derive(PartialEq, Properties)]
pub struct SchrankDetailsComponentProps {
    pub schrank: Schrank,
    pub modal_state: UseStateHandle<ModalState>,
}

//TODO: refactor to only use a single request

#[component]
pub fn SchrankDetailsComponent(
    SchrankDetailsComponentProps {
        schrank,
        modal_state,
    }: &SchrankDetailsComponentProps,
) -> HtmlResult {
    let sc_id = schrank.sc_id;
    let switches_deps = use_state(|| false);
    let switches = use_future_with((sc_id, *switches_deps), |deps| async move {
        let sc_id = deps.0;
        util::fetch_get::<Vec<Switch>>(&format!(
            "/api/switch/schrank/{}",
            urlencoding::encode(&sc_id.to_string())
        ))
        .await
        .unwrap_or_default()
    })?;

    let modal_state_clone = modal_state.clone();
    let schrank_clone = schrank.clone();
    let switches_deps_clone = switches_deps.clone();
    let on_add_switch_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::AddSwitch(
            schrank_clone.clone(),
            switches_deps_clone.clone(),
        ));
    });

    let switches_ref = use_node_ref();

    Ok(html! {
        <div id="switches" ref={switches_ref.clone()}>
            for switch in switches.iter().cloned() {
                <SwitchComponent switch={switch} switches_deps={switches_deps.clone()} modal_state={modal_state.clone()} />
            }
            <img src="assets/svg/plus.svg" id="addButton" onclick={on_add_switch_button_click} />
        </div>
    })
}
