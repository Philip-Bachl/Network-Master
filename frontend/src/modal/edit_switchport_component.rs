use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{
    Callback, HtmlResult, NodeRef, Properties, UseStateHandle, component, html,
    suspense::use_future, use_node_ref,
};

use crate::{
    ModalState,
    model::{Switch, Switchport},
    util,
};

#[derive(PartialEq, Properties)]
pub struct EditSwitchportComponentProps {
    pub switchport: Switchport,
    pub start_switch: Switch,
    pub switchport_details_deps: UseStateHandle<bool>,
    pub modal_state: UseStateHandle<ModalState>,
}

#[component]
pub fn EditSwitchportComponent(
    EditSwitchportComponentProps {
        switchport,
        start_switch,
        switchport_details_deps,
        modal_state,
    }: &EditSwitchportComponentProps,
) -> HtmlResult {
    /*
       pub sp_id: i32,
       pub sp_sw_name: String,
       pub sp_port: String,
       pub sp_vlan: i32,
       pub sp_dot1x: bool,
       pub sp_kommentar: Option<String>,
    */

    let switch_list = use_future(|| async {
        util::fetch_get::<Vec<Switch>>("api/switch")
            .await
            .unwrap_or_default()
    })?;

    let form_data = FormData {
        switch_name_ref: use_node_ref(),
        port_ref: use_node_ref(),
        vlan_ref: use_node_ref(),
        dot1x_ref: use_node_ref(),
        kommentar_ref: use_node_ref(),
    };

    let sp_id_clone = switchport.sp_id;
    let form_data_clone = form_data.clone();
    let switchport_details_deps_clone = switchport_details_deps.clone();
    let modal_state_clone = modal_state.clone();

    let on_create_button_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(handle_create_button_click(
            sp_id_clone,
            form_data_clone.clone(),
            switchport_details_deps_clone.clone(),
            modal_state_clone.clone(),
        ));
    });

    let modal_state_clone = modal_state.clone();
    let on_cancel_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::Nothing);
    });

    //MEDIUM TODO: add option to connect dose to switchport

    Ok(html! {
        <div id="editSwitchport">
            <select id="switchSelect" ref={form_data.switch_name_ref}>
                for switch in switch_list.iter().cloned() {
                    <option selected={ start_switch.sw_name == switch.sw_name } value={switch.sw_name.clone()}>{switch.sw_name}</option>
                }
            </select>
            <input
                ref={form_data.port_ref}
                placeholder="Port"
                id="switchportPortInput"
                type="text"
                value={switchport.sp_port.clone()}
            />
            <input
                ref={form_data.vlan_ref}
                placeholder="Vlan"
                id="switchportVlanInput"
                type="text"
                pattern="[0-9]+"
                value={switchport.sp_vlan.to_string()}
            />
            <div id="dot1x">
                <label for="dot1x">{ "dot1x:" }</label>
                <input
                    ref={form_data.dot1x_ref}
                    placeholder="Dot1x"
                    id="switchportDot1xInput"
                    type="checkbox"
                    checked={switchport.sp_dot1x}
                    name="dot1x"
                />
            </div>
            <input
                ref={form_data.kommentar_ref}
                placeholder="Optional: Kommentar"
                id="switchportKommentarInput"
                type="text"
                value={ switchport.sp_kommentar.clone() }
            />
            <div id="buttons">
                <input type="button" id="acceptButton" onclick={on_create_button_click} value="Speichern"/>
                <input type="button" id="cancelButton" onclick={on_cancel_button_click} value="Abbrechen"/>
            </div>
        </div>
    })
}

#[derive(Clone)]
struct FormData {
    pub switch_name_ref: NodeRef,
    pub port_ref: NodeRef,
    pub vlan_ref: NodeRef,
    pub dot1x_ref: NodeRef,
    pub kommentar_ref: NodeRef,
}

async fn handle_create_button_click(
    sp_id: i32,
    form_data: FormData,
    switchport_details_deps: UseStateHandle<bool>,
    modal_state: UseStateHandle<ModalState>,
) {
    let Some(switch_name) = form_data
        .switch_name_ref
        .cast::<HtmlSelectElement>()
        .map(|s| s.value())
    else {
        //SMALL TODO: error handling
        return;
    };

    let Some(port) = form_data
        .port_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|v| !v.is_empty())
    else {
        util::alert("Port Feld ist leer");
        return;
    };

    let Some(vlan) = form_data
        .vlan_ref
        .cast::<HtmlInputElement>()
        .filter(|i| i.check_validity())
        .map(|i| i.value())
        .and_then(|v| v.parse::<i32>().ok())
    else {
        util::alert("Vlan Feld ist nicht in der Form: <Zahl>");
        return;
    };

    let Some(dot1x) = form_data
        .dot1x_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.checked())
    else {
        //SMALL TODO: error handling
        return;
    };

    let kommentar = form_data
        .kommentar_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value());

    let update_switchport = Switchport {
        sp_id,
        sp_sw_name: switch_name,
        sp_port: port,
        sp_vlan: vlan,
        sp_dot1x: dot1x,
        sp_kommentar: kommentar,
    };
    let Ok(serialized_update_switch) = serde_json::to_string(&update_switchport) else {
        //SMALL TODO: error handling
        return;
    };

    util::fetch_put_with_body("/api/switchport", serialized_update_switch).await;

    switchport_details_deps.set(!*switchport_details_deps);
    modal_state.set(ModalState::Nothing);
}
