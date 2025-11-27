use crate::{Button, Input, Textarea};
use leptos::prelude::*;

#[component]
pub fn InputGroup(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
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
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] button_type: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] variant: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <Button
            button_type=button_type.get_untracked().unwrap_or("button".to_string())
            class=format!(
                "singlestage-input-group-button {} {}",
                match size.get_untracked().unwrap_or_default().as_str() {
                    "sm" => "singlestage-input-group-button-sm",
                    "icon-xs" => "singlestage-input-group-button-icon-xs",
                    "icon-sm" => "singlestage-input-group-button-icon-sm",
                    _ => "singlestage-input-group-button-xs",
                },
                class.get().unwrap_or_default(),
            )
            disabled=disabled.get_untracked()
            size=size.get_untracked().unwrap_or("xs".to_string())
            variant=variant.get_untracked().unwrap_or("ghost".to_string())
        >
            {children()}
        </Button>
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
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] input_type: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] readonly: MaybeProp<bool>,
    #[prop(optional, into)] title: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <Input
            attr:aria_label=aria_label.get_untracked()
            class=format!(
                "singlestage-input-group-input {}",
                class.get_untracked().unwrap_or_default(),
            )
            attr:data-slot="input-group-control"
            id=id.get_untracked()
            input_type=input_type.get_untracked()
            placeholder=placeholder.get_untracked()
            readonly=readonly.get_untracked()
            title=title.get_untracked()
        />
    }
}

#[component]
pub fn InputGroupTextarea(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <Textarea
            class=format!(
                "singlestage-input-group-textarea {}",
                class.get_untracked().unwrap_or_default(),
            )
            attr:data-slot="input-group-control"
            id=id.get_untracked().unwrap_or_default()
            placeholder=placeholder.get_untracked().unwrap_or_default()
        />
    }
}
