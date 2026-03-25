use yew::{Html, Properties, UseStateHandle, component, html};

use crate::{
    ModalState,
    modal::{
        add_dose_component::AddDoseComponent, add_gebaeude_component::AddGebaeudeComponent,
        add_raum_component::AddRaumComponent, add_schrank_component::AddSchrankComponent,
        add_switch_component::AddSwitchComponent, edit_dose_component::EditDoseComponent,
        edit_switchport_component::EditSwitchportComponent,
    },
};

#[derive(PartialEq, Properties)]
pub struct ModalComponentProps {
    pub modal_state: UseStateHandle<ModalState>,
}

//SMALL TODO: Refactor to use a more central approach for modals

#[component]
pub fn ModalComponent(ModalComponentProps { modal_state }: &ModalComponentProps) -> Html {
    let content = match **modal_state {
        ModalState::AddSwitch(ref schrank, ref switches_deps) => {
            html! {
                <AddSwitchComponent modal_state={modal_state} start_schrank={schrank.clone()} switches_deps={switches_deps.clone()} />
            }
        }
        ModalState::AddDose(ref raum, ref dosen_deps) => html! {
            <AddDoseComponent modal_state={modal_state} start_raum={raum.clone()} dosen_deps={dosen_deps.clone()}/>
        },
        ModalState::EditSwitchport(
            ref switchport,
            ref start_switch,
            ref switchport_details_deps,
        ) => html! {
            <EditSwitchportComponent modal_state={modal_state} start_switch={start_switch.clone()} switchport={switchport.clone()} switchport_details_deps={switchport_details_deps.clone()} />
        },
        ModalState::EditDose(ref dose, ref raum, ref switchport, ref dosen_details_ref) => html! {
            <EditDoseComponent dose={dose.clone()} start_raum={raum.clone()} start_switchport={switchport.clone()} dosen_details_deps={dosen_details_ref.clone()} modal_state={modal_state.clone()} />
        },
        ModalState::AddGebaeude(ref gebaeude_deps) => html! {
            <AddGebaeudeComponent modal_state={modal_state.clone()} gebaeude_deps={gebaeude_deps.clone()} />
        },
        ModalState::AddRaum(ref raeume_deps) => html! {
            <AddRaumComponent modal_state={modal_state.clone()} raeume_deps={raeume_deps.clone()} start_gebaeude={None} /> //SMALL TODO: read currently selected value (if any) of sidebar and use that as start_gebaeude
        },
        ModalState::AddSchrank(ref schraenke_deps) => html! {
            <AddSchrankComponent modal_state={modal_state.clone()} schraenke_deps={schraenke_deps.clone()} start_gebaeude={None} /> //SMALL TODO: read currently selected value (if any) of sidebar and use that as start_gebaeude
        },
        ModalState::Nothing => {
            return html! {};
        }
    };

    html! {
        <div id="modal">
            {content}
        </div>
    }
}
