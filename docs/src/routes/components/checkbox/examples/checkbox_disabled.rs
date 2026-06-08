use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxDisabledExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-56">
            <Field orientation="horizontal" disabled=true>
                <Checkbox>"Enable notifications"</Checkbox>
            </Field>

            <Checkbox disabled=true>"Enable notifications"</Checkbox>
        </FieldGroup>
    }
}
