use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldGroupExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldGroup>
                <FieldSet class="gap-3">
                    <FieldLegend variant="label">"Responses"</FieldLegend>
                    <FieldDescription>
                        "Get notified when ChatGPT responds to requests that take time, like
                        research or image generation."
                    </FieldDescription>
                    <Field orientation="horizontal">
                        <Checkbox checked=true class="font-normal" disabled=true>
                            "Push notifications"
                        </Checkbox>
                    </Field>
                </FieldSet>
                <FieldSeparator />
                <FieldSet>
                    <FieldLegend variant="label">"Tasks"</FieldLegend>
                    <FieldDescription>
                        "Get notified when tasks you've created have updates. "
                        <a href="#">"Manage tasks"</a>
                    </FieldDescription>
                    <FieldGroup>
                        <CheckboxGroup>
                            <Field orientation="horizontal">
                                <Checkbox class="font-normal">"Push notifications"</Checkbox>
                            </Field>
                            <Field orientation="horizontal">
                                <Checkbox class="font-normal">"Email notifications"</Checkbox>
                            </Field>
                        </CheckboxGroup>
                    </FieldGroup>
                </FieldSet>
            </FieldGroup>
        </div>
    }
}
