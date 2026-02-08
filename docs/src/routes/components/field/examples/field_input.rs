use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldInputExample() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-xs">
            <FieldGroup>
                <Field>
                    <Label label_for="username">"Username"</Label>
                    <Input id="username" input_type="text" placeholder="Max Leiter" />
                    <FieldDescription>
                        "Choose a unique username for your account."
                    </FieldDescription>
                </Field>
                <Field>
                    <Label label_for="password">"Password"</Label>
                    <FieldDescription>"Must be at least 8 characters long."</FieldDescription>
                    <Input
                        id="password"
                        input_type="password"
                        placeholder="••••••••"
                    />
                </Field>
            </FieldGroup>
        </FieldSet>
    }
}
