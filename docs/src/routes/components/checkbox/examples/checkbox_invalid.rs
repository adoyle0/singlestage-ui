use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxInvalidExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-56">
            <Field orientation="horizontal">
                <Checkbox invalid=true />
                <FieldLabel>"Accept terms and conditions"</FieldLabel>
            </Field>
            <Field orientation="horizontal" invalid=true>
                <Checkbox />
                <FieldLabel>"Accept terms and conditions"</FieldLabel>
            </Field>
        </FieldGroup>
    }
}
