use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxDisabledExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-56">
            <Field orientation="horizontal" disabled=true>
                <Checkbox
                    id="toggle-checkbox-disabled"
                    name="toggle-checkbox-disabled"
                    disabled=true
                />
                <Label label_for="toggle-checkbox-disabled">"Enable notifications"</Label>
            </Field>
        </FieldGroup>
    }
}
