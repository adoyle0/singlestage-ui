use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchDisabledExample() -> impl IntoView {
    view! {
        <Field orientation="horizontal" disabled=true class="w-fit">
            <Switch id="switch-disabled-unchecked" />
            <FieldLabel label_for="switch-disabled-unchecked">"Disabled"</FieldLabel>
        </Field>
    }
}
