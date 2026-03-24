use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{
    Callback, HtmlResult, NodeRef, Properties, TargetCast, UseStateHandle, component, html,
    suspense::{use_future, use_future_with},
    use_node_ref, use_state_eq,
};

use crate::{
    ModalState,
    model::{DeviceKind, Dose, Gebaeude, Raum, Switch, Switchport},
    util,
};

#[derive(PartialEq, Properties)]
pub struct EditDoseComponentProps {
    pub dose: Dose,
    pub start_raum: Raum,
    pub start_switchport: Option<Switchport>,
    pub dosen_details_deps: UseStateHandle<bool>, //TODO: make consistant: either plural or singular everywhere
    pub modal_state: UseStateHandle<ModalState>,
}

#[component]
pub fn EditDoseComponent(
    EditDoseComponentProps {
        dose,
        start_raum,
        start_switchport,
        dosen_details_deps,
        modal_state,
    }: &EditDoseComponentProps,
) -> HtmlResult {
    let gebaeude_list = use_future(|| async move {
        util::fetch_get::<Vec<Gebaeude>>("/api/gebaeude")
            .await
            .unwrap_or_default()
    })?;
    let selected_gebaeude_name = use_state_eq(|| start_raum.ra_ge_name.clone());

    let raum_list = use_future_with(selected_gebaeude_name.clone(), |ge_name| async move {
        if ge_name.is_empty() {
            return vec![];
        }
        util::fetch_get::<Vec<Raum>>(&format!(
            "/api/raum/gebaeude/{}",
            urlencoding::encode(&ge_name)
        ))
        .await
        .unwrap_or_default()
    })?;
    let selected_raum_id = use_state_eq(|| start_raum.ra_id);

    let switch_list = use_future_with((*selected_gebaeude_name).clone(), |ge_name| async move {
        if ge_name.is_empty() {
            return vec![];
        }

        util::fetch_get::<Vec<Switch>>(&format!(
            "/api/switch/gebaeude/{}",
            urlencoding::encode(&ge_name)
        ))
        .await
        .unwrap_or_default()
    })?;
    let selected_switch_name = use_state_eq(|| {
        start_switchport
            .clone()
            .map(|sp| sp.sp_sw_name)
            .unwrap_or_default()
    });

    let switchport_list = use_future_with((*selected_switch_name).clone(), |sw_name| async move {
        if sw_name.is_empty() {
            return vec![];
        }

        util::fetch_get::<Vec<Switchport>>(&format!(
            "/api/switchport/switch/{}",
            urlencoding::encode(&sw_name)
        ))
        .await
        .unwrap_or_default()
    })?;
    let selected_switchport_id = use_state_eq(|| start_switchport.clone().map(|ssp| ssp.sp_id));

    let device_kind_list = use_future(|| async move {
        util::fetch_get::<Vec<DeviceKind>>("/api/device_kind")
            .await
            .unwrap_or_default()
    })?;
    let selected_device_kind_id = use_state_eq(|| dose.do_dk_id);

    //TODO: add way to add device kinds

    let selected_gebaeude_name_clone = selected_gebaeude_name.clone();
    let on_select_gebaeude = Callback::from(move |event: yew::Event| {
        let select: HtmlSelectElement = event.target_unchecked_into();
        selected_gebaeude_name_clone.set(select.value());
    });

    let selected_raum_id_clone = selected_raum_id.clone();
    let start_raum_id_clone = start_raum.ra_id;
    let on_select_raum = Callback::from(move |event: yew::Event| {
        let select: HtmlSelectElement = event.target_unchecked_into();
        selected_raum_id_clone.set(select.value().parse::<i32>().unwrap_or(start_raum_id_clone));
    });

    let selected_switch_name_clone = selected_switch_name.clone();
    let on_select_switch = Callback::from(move |event: yew::Event| {
        let select: HtmlSelectElement = event.target_unchecked_into();
        selected_switch_name_clone.set(select.value());
    });

    let selected_switchport_id_clone = selected_switchport_id;
    let on_select_switchport = Callback::from(move |event: yew::Event| {
        let select: HtmlSelectElement = event.target_unchecked_into();
        selected_switchport_id_clone.set(select.value().parse::<i32>().ok());
    });

    let selected_device_kind_id_clone = selected_device_kind_id.clone();
    let on_select_device_kind = Callback::from(move |event: yew::Event| {
        let select: HtmlSelectElement = event.target_unchecked_into();
        selected_device_kind_id_clone.set(select.value().parse::<i32>().ok());
    });

    let form_data = FormData {
        dose_raum_select_ref: use_node_ref(),
        dose_switchport_select_ref: use_node_ref(),
        dose_device_kind_select_ref: use_node_ref(),
        dose_nummer_ref: use_node_ref(),
        dose_kommentar_ref: use_node_ref(),
    };
    let form_data_clone = form_data.clone();

    let do_id = dose.do_id;
    let dosen_details_deps_clone = dosen_details_deps.clone();
    let modal_state_clone = modal_state.clone();
    let on_create_button_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(handle_create_button_click(
            do_id,
            form_data_clone.clone(),
            dosen_details_deps_clone.clone(),
            modal_state_clone.clone(),
        ));
    });

    let modal_state_clone = modal_state.clone();
    let on_cancel_button_click = Callback::from(move |_| {
        modal_state_clone.set(ModalState::Nothing);
    });

    Ok(html! {
        <div id="editDose">
            <select id="gebaeudeSelect" onchange={on_select_gebaeude}>
                for gebaeude in gebaeude_list.iter().cloned() {
                    <option selected={ *selected_gebaeude_name == gebaeude.ge_name } value={gebaeude.ge_name.clone()}>{gebaeude.ge_name}</option>
                    //TODO: ^^^ optimization for cloning (i know, but you need this a couple of times to not forget!)
                }
            </select>
            //TODO: always use a use_state to keep track of selected items everywhere. otherwise a reload of any kind voids their state
            <select id="raumSelect" onchange={on_select_raum} ref={form_data.dose_raum_select_ref}>
                for raum in raum_list.iter().cloned() {
                    <option selected={ *selected_raum_id == raum.ra_id } value={raum.ra_id.to_string()}>{raum.ra_nummer}</option>
                }
            </select>
            //TODO: add schrank select (maybe)
            <select id="switchSelect" onchange={on_select_switch}>
                <option selected={ dose.do_sp_id.is_none() } value={""}>{"<Switch>"}</option>

                for switch in switch_list.iter().cloned() {
                    <option selected={ start_switchport.clone().map(|sp| sp.sp_sw_name).unwrap_or_default() == switch.sw_name } value={switch.sw_name.clone()}>{switch.sw_name}</option>
                }
            </select>
            <select id="switchportSelect" ref={form_data.dose_switchport_select_ref} onchange={on_select_switchport}>
                <option selected={ dose.do_sp_id.is_none() } value={""}>{"<Kein Port verbunden>"}</option>

                for switchport in switchport_list.iter().cloned() {
                    <option selected={ start_switchport.clone().map(|sp| sp.sp_id).unwrap_or_default() == switchport.sp_id } value={switchport.sp_id.to_string()}>{switchport.sp_port}</option>
                }
            </select>
            <select id="deviceKindSelect" ref={form_data.dose_device_kind_select_ref} onchange={on_select_device_kind}>
                <option selected={ dose.do_dk_id.is_none() } value={""}>{"<Kein Gerät verbunden>"}</option>
                for device_kind in device_kind_list.iter().cloned() {
                    <option selected={ dose.do_dk_id == Some(device_kind.dk_id) } value={device_kind.dk_id.to_string()}>{device_kind.dk_name}</option>
                    //TODO: find a way to display icons instead of text (option elements dont allow anything but text, probably have to create custom select)
                }
            </select>
            <input
                type="text"
                id="doseNummerInput"
                placeholder="Dosennummer"
                ref={form_data.dose_nummer_ref}
                value={dose.do_nummer.clone()}
            />
            <input
                type="text"
                id="doseKommentarInput"
                placeholder="Optional: Kommentar" //TODO: change "Optional: ..." to "... (Optional)"
                ref={form_data.dose_kommentar_ref}
                value={dose.do_kommentar.clone()}
            />
            <div id="buttons">
                <input type="button" id="acceptButton" onclick={on_create_button_click} value="Speichern"/> //TODO: change "create..." stuff to "apply" or "save" or something
                <input type="button" id="cancelButton" onclick={on_cancel_button_click} value="Abbrechen"/>
            </div>
        </div>
    })
}

#[derive(Clone)]
struct FormData {
    dose_raum_select_ref: NodeRef,
    dose_switchport_select_ref: NodeRef,
    dose_device_kind_select_ref: NodeRef,
    dose_nummer_ref: NodeRef,
    dose_kommentar_ref: NodeRef,
} //TODO: make the dose_ prefix naming of stuff inside FormData consistant everywhere (either have it or not)

async fn handle_create_button_click(
    do_id: i32,
    form_data: FormData,
    dosen_details_deps: UseStateHandle<bool>,
    modal_state: UseStateHandle<ModalState>,
) {
    //TODO: maybe use references here and at all other create_... spots instead of owned data (reduces cloning)
    let Some(raum_id) = form_data
        .dose_raum_select_ref
        .cast::<HtmlSelectElement>()
        .map(|s| s.value())
        .and_then(|v| v.parse::<i32>().ok())
    else {
        //TODO: error handling
        return;
    };

    let switchport_id = form_data
        .dose_switchport_select_ref
        .cast::<HtmlSelectElement>()
        .map(|s| s.value())
        .filter(|v| !v.is_empty())
        .and_then(|v| v.parse::<i32>().ok());
    let device_kind_id = form_data
        .dose_device_kind_select_ref
        .cast::<HtmlSelectElement>()
        .map(|s| s.value())
        .filter(|v| !v.is_empty())
        .and_then(|v| v.parse::<i32>().ok());

    let Some(dose_nummer) = form_data
        .dose_nummer_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|v| !v.is_empty())
    else {
        //TODO: error handling
        return;
    };

    let dose_kommentar = form_data
        .dose_kommentar_ref
        .cast::<HtmlInputElement>()
        .map(|i| i.value())
        .filter(|v| !v.is_empty());

    let dose = Dose {
        do_id,
        do_ra_id: raum_id,
        do_nummer: dose_nummer,
        do_sp_id: switchport_id,
        do_dk_id: device_kind_id,
        do_kommentar: dose_kommentar,
    };
    let Ok(serialized_dose) = serde_json::to_string(&dose) else {
        //TODO: error handling
        return;
    };

    util::fetch_put_with_body("/api/dose", serialized_dose).await;
    dosen_details_deps.set(!*dosen_details_deps);
    modal_state.set(ModalState::Nothing);
}
