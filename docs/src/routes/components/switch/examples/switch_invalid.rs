use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchInvalidExample() -> impl IntoView {
    view! {
        <Field orientation="horizontal" class="max-w-sm" invalid=true>
            <FieldContent>
                <FieldLabel label_for="switch-terms">"Accept terms and conditions"</FieldLabel>
                <FieldDescription>
                    "You must accept the terms and conditions to continue."
                </FieldDescription>
            </FieldContent>
            <Switch id="switch-terms" />
        </Field>
    }
}
