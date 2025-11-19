use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldInputExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldGroup>
                    <Field>
                        <FieldLabel label_for="username">"Username"</FieldLabel>
                        <Input id="username" input_type="text" placeholder="Max Leiter" />
                        <FieldDescription>
                            "Choose a unique username for your account."
                        </FieldDescription>
                    </Field>
                    <Field>
                        <FieldLabel label_for="password">"Password"</FieldLabel>
                        <FieldDescription>"Must be at least 8 characters long."</FieldDescription>
                        <Input
                            id="password"
                            input_type="password"
                            placeholder="••••••••"
                        />
                    </Field>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
