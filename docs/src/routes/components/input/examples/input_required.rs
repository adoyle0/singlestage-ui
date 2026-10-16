use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputRequiredExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs" required=true>
            <Input placeholder="This field is required">"Required Field"</Input>
            <FieldDescription>"This field must be filled out."</FieldDescription>
        </Field>
    }
}
