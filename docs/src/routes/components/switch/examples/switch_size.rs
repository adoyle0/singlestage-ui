use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchSizeExample() -> impl IntoView {
    view! {
        <FieldGroup class="w-fit">
            <Field orientation="horizontal">
                <Switch size="sm">"Small"</Switch>
            </Field>
            <Field orientation="horizontal">
                <Switch size="default">"Default"</Switch>
            </Field>
        </FieldGroup>
    }
}
