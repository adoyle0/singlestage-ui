use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxInvalidExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-56">
            <Checkbox invalid=true>"Accept terms and conditions"</Checkbox>

            // Checkboxes defer to their outer-most parent's invalid state
            <Field orientation="horizontal" invalid=true>
                <Checkbox>"Accept terms and conditions"</Checkbox>
            </Field>

            <CheckboxGroup invalid=true>
                <Field orientation="horizontal">
                    <Checkbox>"Accept terms and conditions"</Checkbox>
                </Field>

                <Checkbox>"Accept terms and conditions"</Checkbox>
            </CheckboxGroup>
        </FieldGroup>
    }
}
