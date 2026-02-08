use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldGroupExample() -> impl IntoView {
    view! {
        <FieldGroup class="w-full max-w-xs">
            <FieldSet>
                <Label>"Responses"</Label>
                <FieldDescription>
                    "Get notified when ChatGPT responds to requests that take time, like
                    research or image generation."
                </FieldDescription>
                <FieldGroup>
                    <CheckboxGroup>
                        <Field orientation="horizontal">
                            <Checkbox id="push" checked=true disabled=true />
                            <Label label_for="push" class="font-normal">
                                "Push notifications"
                            </Label>
                        </Field>
                    </CheckboxGroup>
                </FieldGroup>
            </FieldSet>
            <Separator />
            <FieldSet>
                <Label>Tasks</Label>
                <FieldDescription>
                    "Get notified when tasks you&apos;ve created have updates.{" "}"
                    <a href="#">"Manage tasks"</a>
                </FieldDescription>
                <FieldGroup>
                    <CheckboxGroup>
                        <Field orientation="horizontal">
                            <Checkbox id="push-tasks" />
                            <Label label_for="push-tasks" class="font-normal">
                                "Push notifications"
                            </Label>
                        </Field>
                        <Field orientation="horizontal">
                            <Checkbox id="email-tasks" />
                            <Label label_for="email-tasks" class="font-normal">
                                "Email notifications"
                            </Label>
                        </Field>
                    </CheckboxGroup>
                </FieldGroup>
            </FieldSet>
        </FieldGroup>
    }
}
