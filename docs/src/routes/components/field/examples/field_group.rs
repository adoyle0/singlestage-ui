use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldGroupExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldGroup>
                <FieldSet>
                    <FieldLabel>"Responses"</FieldLabel>
                    <FieldDescription>
                        "Get notified when ChatGPT responds to requests that take time, like
                        research or image generation."
                    </FieldDescription>
                    <FieldGroup>
                        <Field orientation="horizontal">
                            <Checkbox id="push" checked=true disabled=true />
                            <FieldLabel label_for="push" class="font-normal">
                                "Push notifications"
                            </FieldLabel>
                        </Field>
                    </FieldGroup>
                </FieldSet>
                <FieldSeparator />
                <FieldSet>
                    <FieldLabel>"Tasks"</FieldLabel>
                    <FieldDescription>
                        "Get notified when tasks you've created have updates. "
                        <a href="#">"Manage tasks"</a>
                    </FieldDescription>
                    <FieldGroup>
                        <CheckboxGroup>
                            <Field orientation="horizontal">
                                <Checkbox id="push-tasks" />
                                <FieldLabel label_for="push-tasks" class="font-normal">
                                    "Push notifications"
                                </FieldLabel>
                            </Field>
                            <Field orientation="horizontal">
                                <Checkbox id="email-tasks" />
                                <FieldLabel label_for="email-tasks" class="font-normal">
                                    "Email notifications"
                                </FieldLabel>
                            </Field>
                        </CheckboxGroup>
                    </FieldGroup>
                </FieldSet>
            </FieldGroup>
        </div>
    }
}
