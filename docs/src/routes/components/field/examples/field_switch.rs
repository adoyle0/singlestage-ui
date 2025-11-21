use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSwitchExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <Field orientation="horizontal">
                <FieldLabel label_for="2fa">
                    <FieldContent>
                        <FieldTitle>"Multi-factor authentication"</FieldTitle>
                        <FieldDescription>
                            "Enable multi-factor authentication. If you do not have a two-factor
                            device, you can use a one-time code sent to your email."
                        </FieldDescription>
                    </FieldContent>
                </FieldLabel>
                <Switch id="2fa" />
            </Field>
        </div>
    }
}
