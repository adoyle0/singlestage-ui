use leptos::prelude::*;

#[component]
pub fn Empty(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-empty {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}

#[component]
pub fn EmptyHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-empty-header {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}

#[component]
pub fn EmptyMedia(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] variant: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!(
                "{} {}",
                match variant.get().unwrap_or_default().as_str() {
                    "icon" => "singlestage-empty-media-icon",
                    _ => "singlestage-empty-media",
                },
                class.get().unwrap_or_default(),
            )
        }>{children()}</div>
    }
}

#[component]
pub fn EmptyTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-empty-title {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}

#[component]
pub fn EmptyDescription(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-empty-description {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}

#[component]
pub fn EmptyContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-empty-content {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}
