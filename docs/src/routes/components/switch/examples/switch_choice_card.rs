use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchChoiceCardExample() -> impl IntoView {
    view! {
        <FieldGroup class="w-full max-w-sm">
            <FieldLabel label_for="switch-share">
                <Field orientation="horizontal">
                    <FieldContent>
                        <FieldTitle>"Share across devices"</FieldTitle>
                        <FieldDescription>
                            "Focus is shared across devices, and turns off when you leave the
                            app."
                        </FieldDescription>
                    </FieldContent>
                    <Switch id="switch-share" />
                </Field>
            </FieldLabel>
            <FieldLabel label_for="switch-notifications">
                <Field orientation="horizontal">
                    <FieldContent>
                        <FieldTitle>"Enable notifications"</FieldTitle>
                        <FieldDescription>
                            "Receive notifications when focus mode is enabled or disabled."
                        </FieldDescription>
                    </FieldContent>
                    <Switch id="switch-notifications" checked=true />
                </Field>
            </FieldLabel>
        </FieldGroup>
    }
}
