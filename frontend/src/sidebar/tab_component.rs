use yew::{AttrValue, Callback, Html, Properties, component, html, use_state_eq};

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub title: AttrValue,
    pub img_url: AttrValue,
    pub children: Html,
}

#[component]
pub fn TabComponent(
    TabProps {
        title,
        img_url,
        children,
    }: &TabProps,
) -> Html {
    let open = use_state_eq(|| false);

    let open_clone = open.clone();
    let onclick = Callback::from(move |_| {
        open_clone.set(!*open_clone);
    });

    html! {
        <div class={
            if *open {
                "tab open"
            } else {
                "tab"
            }
        }>
            <div class="tab-title"  {onclick}>
                <img class="arrow" src="assets/svg/arrow.svg" />
                <img src={img_url} />
                <div>
                    {title}
                </div>
            </div>
            <div class="tab-content">
                {children}
            </div>
        </div>
    }
}
