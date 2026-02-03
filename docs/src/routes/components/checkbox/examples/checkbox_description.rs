use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxDescriptionExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-72">
            <Field orientation="horizontal">
                <Checkbox id="terms-checkbox-desc" name="terms-checkbox-desc" checked=true />
                <FieldContent>
                    <Label label_for="terms-checkbox-desc">"Accept terms and conditions"</Label>
                    <FieldDescription>
                        "By clicking this checkbox, you agree to the terms and conditions."
                    </FieldDescription>
                </FieldContent>
            </Field>
        </FieldGroup>
    }
}
