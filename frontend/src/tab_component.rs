use yew::{AttrValue, Callback, Html, Properties, component, html, use_state_eq};

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub title: AttrValue,
    pub img: Html,
    pub children: Html,
}

#[component]
pub fn TabComponent(
    TabProps {
        title,
        img,
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
                <svg class="arrow" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                    <path style="fill: var(--color);" d="M297.4 438.6C309.9 451.1 330.2 451.1 342.7 438.6L502.7 278.6C515.2 266.1 515.2 245.8 502.7 233.3C490.2 220.8 469.9 220.8 457.4 233.3L320 370.7L182.6 233.4C170.1 220.9 149.8 220.9 137.3 233.4C124.8 245.9 124.8 266.2 137.3 278.7L297.3 438.7z"/>
                </svg>

                {img.clone()}

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
