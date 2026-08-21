use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldGroupExample() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-xs">
            <FieldGroup>
                <FieldLabel>"Responses"</FieldLabel>
                <FieldDescription>
                    "Get notified when ChatGPT responds to requests that take time, like
                    research or image generation."
                </FieldDescription>
                <Field disabled=true orientation="horizontal">
                    <Checkbox checked=true>"Push notifications"</Checkbox>
                </Field>
            </FieldGroup>
            <FieldSeparator />
            <FieldGroup>
                <FieldLabel>"Tasks"</FieldLabel>
                <FieldDescription>
                    "Get notified when tasks you've created have updates. "
                    <Link href="#">"Manage tasks"</Link>
                </FieldDescription>
                <CheckboxGroup>
                    <Field orientation="horizontal">
                        <Checkbox>"Push notifications"</Checkbox>
                    </Field>
                    <Field orientation="horizontal">
                        <Checkbox>"Email notifications"</Checkbox>
                    </Field>
                </CheckboxGroup>
            </FieldGroup>
        </FieldGroup>
    }
}
