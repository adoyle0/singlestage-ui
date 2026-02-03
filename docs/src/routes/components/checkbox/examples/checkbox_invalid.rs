use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxInvalidExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-56">
            <Field orientation="horizontal">
                <Checkbox invalid=true />
                <Label>"Accept terms and conditions"</Label>
            </Field>
            <Field orientation="horizontal" invalid=true>
                <Checkbox />
                <Label>"Accept terms and conditions"</Label>
            </Field>
        </FieldGroup>
    }
}
