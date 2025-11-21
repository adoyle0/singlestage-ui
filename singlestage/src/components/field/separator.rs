use leptos::prelude::*;

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
