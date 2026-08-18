use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxDisabledExample() -> impl IntoView {
    view! {
        <FieldGroup class="mx-auto w-56">
            <Checkbox disabled=true>"Enable notifications"</Checkbox>

            // Checkboxes defer to their outer-most parent's disabled state
            <Field orientation="horizontal" disabled=true>
                <Checkbox>"Enable notifications"</Checkbox>
            </Field>

            <CheckboxGroup disabled=true>
                <Field orientation="horizontal">
                    <Checkbox>"Enable notifications"</Checkbox>
                </Field>

                <Checkbox>"Enable notifications"</Checkbox>
            </CheckboxGroup>
        </FieldGroup>
    }
}
