use yew::{AttrValue, Callback, Html, MouseEvent, Properties, component, html, use_state_eq};

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub title: AttrValue,
    pub img_url: AttrValue,
    pub delete_callback: Option<Callback<MouseEvent>>,
    pub children: Html,
}

#[component]
pub fn TabComponent(
    TabProps {
        title,
        img_url,
        delete_callback,
        children,
    }: &TabProps,
) -> Html {
    let open = use_state_eq(|| false);

    let open_clone = open.clone();
    let onclick = Callback::from(move |_| {
        open_clone.set(!*open_clone);
    });

    let delete_button_html = if delete_callback.is_some() {
        html! {
            <img src="assets/svg/plus.svg" class="delete-button" onclick={delete_callback.to_owned()} />
        }
    } else {
        html! {}
    };

    html! {
        <div class={
            if *open {
                "tab open"
            } else {
                "tab"
            } //MEDIUM TODO: refactor class assignments to use the macro instead everywhere
        }>
            <div class="tab-title"  {onclick}>
                <img class="arrow" src="assets/svg/arrow.svg" />
                <img src={img_url} />
                <div>
                    {title}
                </div>
                {delete_button_html}
            </div>
            <div class="tab-content">
                {children}
            </div>
        </div>
    }
}
