use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxBasicExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-56">
            <Field orientation="horizontal">
                <Checkbox id="terms-checkbox-basic" name="terms-checkbox-basic" />
                <FieldLabel label_for="terms-checkbox-basic">
                    "Accept terms and conditions"
                </FieldLabel>
            </Field>
        </FieldGroup>
    }
}
