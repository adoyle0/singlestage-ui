use leptos::prelude::*;

#[component]
pub fn ScrollArea(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] orientation: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!(
                "singlestage-scroll-area {} {}",
                match orientation.get().unwrap_or_default().as_str() {
                    "horizontal" => "singlestage-scroll-area-horizontal",
                    _ => "singlestage-scroll-area-vertical",
                },
                class.get().unwrap_or_default(),
            )
        }>{children()}</div>
    }
}

#[component]
pub fn ScrollAreaScrollbar(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] orientation: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!(
                "{} {}",
                match orientation.get().unwrap_or_default().as_str() {
                    "horizontal" => "singlestage-scroll-area-horizontal",
                    _ => "singlestage-scroll-area-vertical",
                },
                class.get().unwrap_or_default(),
            )
        }></div>
    }
}
