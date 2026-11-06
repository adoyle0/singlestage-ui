use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGridExample() -> impl IntoView {
    view! {
        <FieldGroup class="grid max-w-sm grid-cols-2">
            <Field>
                <Input placeholder="Jordan">"First Name"</Input>
            </Field>
            <Field>
                <Input placeholder="Lee">"Last Name"</Input>
            </Field>
        </FieldGroup>
    }
}
