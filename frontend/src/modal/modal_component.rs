use yew::{Html, Properties, UseStateHandle, component, html};

use crate::{ModalState, modal::add_switch_component::AddSwitchComponent};

#[derive(PartialEq, Properties)]
pub struct ModalComponentProps {
    pub modal_state: UseStateHandle<ModalState>,
}

#[component]
pub fn ModalComponent(ModalComponentProps { modal_state }: &ModalComponentProps) -> Html {
    let content = match **modal_state {
        ModalState::AddSwitch(ref schrank, ref switches_deps) => html! {
            <AddSwitchComponent modal_state={modal_state} start_schrank={schrank.clone()} switches_deps={switches_deps.clone()}/>
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
