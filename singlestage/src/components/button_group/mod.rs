use leptos::prelude::*;

#[component]
pub fn ButtonGroup(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] orientation: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            role="group"
            data-slot="button-group"
            data-orientation="horizontal"
            class=move || {
                format!(
                    "singlestage-button-group {} {}",
                    match orientation.get().unwrap_or_default().as_str() {
                        "vertical" => "singlestage-button-group-vertical",
                        _ => "singlestage-button-group-horizontal",
                    },
                    class.get().unwrap_or_default(),
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ButtonGroupText(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-button-group-text {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}

#[component]
pub fn ButtonGroupSeparator(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] orientation: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            data-slot="button-group-separator"
            data-orientation=move || orientation.get().unwrap_or("vertical".to_string())
            class=move || {
                format!(
                    "singlestage-separator singlestage-button-group-separator {}",
                    class.get().unwrap_or_default(),
                )
            }
        ></div>
    }
}
