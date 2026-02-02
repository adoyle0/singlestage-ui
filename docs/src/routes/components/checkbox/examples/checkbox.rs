use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxExample() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-sm">
            <Field orientation="horizontal">
                <Checkbox id="terms-checkbox" name="terms-checkbox" />
                <Label label_for="terms-checkbox">"Accept terms and conditions"</Label>
            </Field>
            <Field orientation="horizontal">
                <Checkbox id="terms-checkbox-2" name="terms-checkbox-2" checked=true />
                <FieldContent>
                    <FieldLabel label_for="terms-checkbox-2">
                        "Accept terms and conditions"
                    </FieldLabel>
                    <FieldDescription>
                        "By clicking this checkbox, you agree to the terms."
                    </FieldDescription>
                </FieldContent>
            </Field>
            <Field orientation="horizontal" disabled=true>
                <Checkbox id="toggle-checkbox" name="toggle-checkbox" disabled=true />
                <FieldLabel label_for="toggle-checkbox">"Enable notifications"</FieldLabel>
            </Field>
            <FieldLabel>
                <Field orientation="horizontal">
                    <Checkbox id="toggle-checkbox-2" name="toggle-checkbox-2" />
                    <FieldContent>
                        <FieldTitle>"Enable notifications"</FieldTitle>
                        <FieldDescription>
                            "You can enable or disable notifications at any time."
                        </FieldDescription>
                    </FieldContent>
                </Field>
            </FieldLabel>
        </FieldGroup>
    }
}
