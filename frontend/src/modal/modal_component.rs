use yew::{Html, Properties, UseStateHandle, component, html};

use crate::{
    ModalState,
    modal::{
        add_dose_component::AddDoseComponent, add_switch_component::AddSwitchComponent,
        edit_dose_component::EditDoseComponent, edit_switchport_component::EditSwitchportComponent,
    },
};

#[derive(PartialEq, Properties)]
pub struct ModalComponentProps {
    pub modal_state: UseStateHandle<ModalState>,
}

//TODO: Refactor to use a more central approach for modals

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
            ref start_switch,
            ref switchport,
            ref switchport_details_deps,
        ) => html! {
            <EditSwitchportComponent modal_state={modal_state} start_switch={start_switch.clone()} switchport={switchport.clone()} switchport_details_deps={switchport_details_deps.clone()} />
        },
        ModalState::EditDose(ref dose, ref raum, ref switchport, ref dosen_details_ref) => html! {
            <EditDoseComponent dose={dose.clone()} start_raum={raum.clone()} start_switchport={switchport.clone()} dosen_details_deps={dosen_details_ref.clone()} modal_state={modal_state.clone()} />
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
