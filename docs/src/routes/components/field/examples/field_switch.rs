use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSwitchExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <Field orientation="horizontal">
                <FieldContent>
                    <FieldLabel label_for="2fa">"Multi-factor authentication"</FieldLabel>
                    <FieldDescription>
                        "Enable multi-factor authentication. If you do not have a two-factor
                        device, you can use a one-time code sent to your email."
                    </FieldDescription>
                </FieldContent>
                <Switch id="2fa" />
            </Field>
        </div>
    }
}
