use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxBasicExample() -> impl IntoView {
    view! {
        <FieldGroup class="w-56">
            <Field orientation="horizontal">
                <Checkbox>"Accept terms and conditions"</Checkbox>
            </Field>

            <Field orientation="horizontal">
                <Checkbox />
                <FieldLabel>"Accept terms and conditions"</FieldLabel>
            </Field>

            <Field orientation="horizontal">
                <Checkbox id="terms-checkbox-basic" />
                <FieldLabel label_for="terms-checkbox-basic">
                    "Accept terms and conditions"
                </FieldLabel>
            </Field>
        </FieldGroup>
    }
}
