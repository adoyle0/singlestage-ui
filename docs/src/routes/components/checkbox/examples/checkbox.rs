use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxExample() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-sm">
            <Field orientation="horizontal">
                <Checkbox>"Accept terms and conditions"</Checkbox>
            </Field>

            <Field orientation="horizontal">
                <Checkbox checked=true />
                <FieldContent>
                    <FieldLabel>"Accept terms and conditions"</FieldLabel>
                    <FieldDescription>
                        "By clicking this checkbox, you agree to the terms."
                    </FieldDescription>
                </FieldContent>
            </Field>

            <Field orientation="horizontal" disabled=true>
                <Checkbox>"Enable notifications"</Checkbox>
            </Field>

            <FieldLabel>
                <Field orientation="horizontal">
                    <Checkbox />
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
