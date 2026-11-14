use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputFieldExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <Input placeholder="Enter your username">"Username"</Input>
            <FieldDescription>"Choose a unique username for your account."</FieldDescription>
        </Field>
    }
}
