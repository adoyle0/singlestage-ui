use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSwitchExample() -> impl IntoView {
    view! {
        <Field orientation="horizontal" class="w-fit">
            <FieldLabel label_for="2fa">"Multi-factor authentication"</FieldLabel>
            <Switch id="2fa" />
        </Field>
    }
}
