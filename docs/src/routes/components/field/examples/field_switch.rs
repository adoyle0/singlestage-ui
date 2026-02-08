use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSwitchExample() -> impl IntoView {
    view! {
        <Field orientation="horizontal" class="w-fit">
            <Label label_for="2fa">"Multi-factor authentication"</Label>
            <Switch id="2fa" />
        </Field>
    }
}
