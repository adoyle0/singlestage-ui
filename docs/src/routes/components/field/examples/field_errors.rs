use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldErrorsExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldGroup>
                    <Field>
                        <FieldLabel>"Username"</FieldLabel>
                        <FieldError>"Username is required"</FieldError>
                        <Input invalid=true placeholder="Max Leiter" />
                    </Field>
                    <Field>
                        <FieldLabel>"Password"</FieldLabel>
                        <FieldError errors=vec![
                            "Password must be 8 characters long".to_string(),
                            "Password must contain both uppercase and lowercase letters"
                                .to_string(),
                            "Password must contain a number".to_string(),
                        ] />
                        <Input
                            input_type="password"
                            invalid=true
                            placeholder="••••••••"
                        />
                    </Field>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
