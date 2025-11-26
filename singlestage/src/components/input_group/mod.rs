use crate::{Input, Textarea};
use leptos::prelude::*;

#[component]
pub fn InputGroup(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            class=move || { format!("singlestage-input-group {}", class.get().unwrap_or_default()) }
            role="group"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn InputGroupAddon(
    children: Children,
    #[prop(optional, into)] align: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                format!(
                    "singlestage-input-group-addon {} {}",
                    match align.get().unwrap_or_default().as_str() {
                        "inline-end" => "singlestage-input-group-addon-inline-end",
                        "block-start" => "singlestage-input-group-addon-block-start",
                        "block-end" => "singlestage-input-group-addon-block-end",
                        _ => "singlestage-input-group-addon-inline-start",
                    },
                    class.get().unwrap_or_default(),
                )
            }
            role="group"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn InputGroupButton(
    children: Children,
    #[prop(optional, into)] button_type: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] variant: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                format!(
                    "{} {} {} singlestage-input-group-button",
                    match variant.get().unwrap_or_default().as_str() {
                        "primary" => "singlestage-btn-primary",
                        "secondary" => "singlestage-btn-secondary",
                        "outline" => "singlestage-btn-outline",
                        "ghost" => "singlestage-btn-ghost",
                        "link" => "singlestage-btn-link",
                        "destructive" => "singlestage-btn-destructive",
                        _ => "singlestage-btn-primary",
                    },
                    match size.get().unwrap_or_default().as_str() {
                        "sm" => "singlestage-btn-sm",
                        "small" => "singlestage-btn-sm",
                        "lg" => "singlestage-btn-lg",
                        "large" => "singlestage-btn-lg",
                        "icon" => "singlestage-btn-icon",
                        "sm-icon" => "singlestage-btn-sm-icon",
                        "icon-sm" => "singlestage-btn-sm-icon",
                        "lg-icon" => "singlestage-btn-lg-icon",
                        "icon-lg" => "singlestage-btn-lg-icon",
                        _ => "",
                    },
                    class.get().unwrap_or_default(),
                )
            }
            disabled=disabled.get_untracked()
            prop:disabled=move || disabled.get()
            type=button_type.get()
        >
            {children()}
        </button>
    }
}

#[component]
pub fn InputGroupText(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <span class=move || {
            format!("singlestage-input-group-text {}", class.get().unwrap_or_default())
        }>{children()}</span>
    }
}

#[component]
pub fn InputGroupInput(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] input_type: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <Input
            class=format!(
                "singlestage-input-group-input {}",
                class.get_untracked().unwrap_or_default(),
            )
            id=id.get_untracked().unwrap_or_default()
            input_type=input_type.get_untracked().unwrap_or_default()
            placeholder=placeholder.get_untracked().unwrap_or_default()
        />
    }
}

#[component]
pub fn InputGroupTextarea(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <Textarea
            class=format!(
                "singlestage-input-group-textarea {}",
                class.get_untracked().unwrap_or_default(),
            )
            id=id.get_untracked().unwrap_or_default()
            placeholder=placeholder.get_untracked().unwrap_or_default()
        />
    }
}
