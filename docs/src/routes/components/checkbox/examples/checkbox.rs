use leptos::prelude::*;
use singlestage::checkbox::*;

#[component]
pub fn CheckboxExample() -> impl IntoView {
    view! { <Checkbox>"Accept terms and conditions"</Checkbox> }
}
