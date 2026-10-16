use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchSizeExample() -> impl IntoView {
    view! {
        <FieldGroup class="w-full max-w-[10rem]">
            <Field orientation="horizontal">
                <Switch id="switch-size-sm" size="sm" />
                <FieldLabel label_for="switch-size-sm">"Small"</FieldLabel>
            </Field>
            <Field orientation="horizontal">
                <Switch id="switch-size-default" size="default" />
                <FieldLabel label_for="switch-size-default">"Default"</FieldLabel>
            </Field>
        </FieldGroup>
    }
}
