use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldInputExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldGroup>
                    <Field>
                        <Input placeholder="Max Leiter">"Username"</Input>
                        <FieldDescription>
                            "Choose a unique username for your account."
                        </FieldDescription>
                    </Field>
                    <Field>
                        <FieldLabel>"Password"</FieldLabel>
                        <FieldDescription>"Must be at least 8 characters long."</FieldDescription>
                        <Input input_type="password" placeholder="••••••••" />
                    </Field>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
