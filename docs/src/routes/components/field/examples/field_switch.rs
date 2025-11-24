use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSwitchExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldGroup>
                    <Field orientation="horizontal">
                        <FieldLabel>
                            <FieldContent>
                                <FieldTitle>"Multi-factor authentication"</FieldTitle>
                                <FieldDescription>
                                    "Enable multi-factor authentication. If you do not have a two-factor
                                    device, you can use a one-time code sent to your email."
                                </FieldDescription>
                            </FieldContent>
                        </FieldLabel>
                        <Switch />
                    </Field>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
