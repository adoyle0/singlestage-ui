use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSwitchExample() -> impl IntoView {
    view! {
        <Field orientation="horizontal" class="w-fit">
            <FieldLabel>"Multi-factor authentication"</FieldLabel>
            <Switch />
        </Field>
    }
}
