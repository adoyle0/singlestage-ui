use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxInvalidExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-56">
            <Field orientation="horizontal" invalid=true>
                <Checkbox>"Accept terms and conditions"</Checkbox>
            </Field>

            <Checkbox invalid=true>"Accept terms and conditions"</Checkbox>
        </FieldGroup>
    }
}
