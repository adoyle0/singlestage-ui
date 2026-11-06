use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputInvalidExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs" invalid=true>
            <Input placeholder="Error">"Invalid Input"</Input>
            <FieldDescription>"This field contains validation errors."</FieldDescription>
        </Field>
    }
}
