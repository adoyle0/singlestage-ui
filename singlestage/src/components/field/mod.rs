use leptos::prelude::*;

#[component]
pub fn Field(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] orientation: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                format!(
                    "singlestage-field {} {}",
                    match orientation.get().unwrap_or_default().as_str() {
                        "horizontal" => "singlestage-field-horizontal",
                        "responsive" => "singlestage-field-responsive",
                        _ => "singlestage-field-vertical",
                    }
                        .to_string(),
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
pub fn FieldDescription(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <p class=move || {
            format!("singlestage-field-description {}", class.get().unwrap_or_default())
        }>{children()}</p>
    }
}

#[component]
pub fn FieldError(children: Children) -> impl IntoView {
    view! { <div>{children()}</div> }
}

#[component]
pub fn FieldGroup(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-field-group {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}

#[component]
pub fn FieldContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-field-content {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}

#[component]
pub fn FieldLegend(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] variant: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <legend class=move || {
            format!(
                "singlestage-field-legend {} {}",
                match variant.get().unwrap_or_default().as_str() {
                    "label" => "singlestage-field-legend-label",
                    _ => "singlestage-field-legend-legend",
                },
                class.get().unwrap_or_default(),
            )
        }>{children()}</legend>
    }
}

#[component]
pub fn FieldSeparator(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-field-separator-outer {}", class.get().unwrap_or_default())
        }>
            <div class="singlestage-field-separator"></div>
            {if let Some(children) = children {
                view! { <span class="singlestage-field-separator-content">{children()}</span> }
                    .into_any()
            } else {
                "".into_any()
            }}
        </div>
    }
}

#[component]
pub fn FieldSet(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <fieldset class=move || {
            format!("singlestage-field-set {}", class.get().unwrap_or_default())
        }>{children()}</fieldset>
    }
}

#[component]
pub fn FieldTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!("singlestage-field-title {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}

#[component]
pub fn FieldLabel(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] label_for: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <label
            class=move || format!("singlestage-field-label {}", class.get().unwrap_or_default())
            for=move || label_for.get().unwrap_or_default()
        >
            {children()}
        </label>
    }
}
