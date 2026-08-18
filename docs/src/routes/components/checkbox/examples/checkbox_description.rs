use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxDescriptionExample() -> impl IntoView {
    view! {
        <FieldGroup class="w-72">
            <Field orientation="horizontal">
                <Checkbox id="terms-checkbox-desc" checked=true />
                <FieldContent>
                    <FieldLabel label_for="terms-checkbox-desc">
                        "Accept terms and conditions"
                    </FieldLabel>
                    <FieldDescription>
                        "By clicking this checkbox, you agree to the terms and conditions."
                    </FieldDescription>
                </FieldContent>
            </Field>
        </FieldGroup>
    }
}
