use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldErrorsExample() -> impl IntoView {
    let username_invalid = RwSignal::new(true);
    let password_invalid = RwSignal::new(true);

    let password_errors = RwSignal::new(vec![
        "Password must be 8 characters long".to_string(),
        "Password must contain a number".to_string(),
        "Password must contain uppercase letters".to_string(),
        "Password must contain lowercase letters".to_string(),
    ]);

    let username_validate = move |ev| {
        if event_target_value(&ev).is_empty() {
            username_invalid.set(true)
        } else {
            username_invalid.set(false)
        }
    };

    let password_validate = move |ev| {
        let password = event_target_value(&ev);

        let mut errors = vec![];

        if password.len() < 8 {
            errors.push("Password must be 8 characters long".to_string());
        };

        if !password.chars().find(|char| char.is_numeric()).is_some() {
            errors.push("Password must contain a number".to_string());
        };

        if !password.chars().find(|char| char.is_uppercase()).is_some() {
            errors.push("Password must contain uppercase letters".to_string());
        };

        if !password.chars().find(|char| char.is_lowercase()).is_some() {
            errors.push("Password must contain lowercase letters".to_string());
        };

        password_invalid.set(!errors.is_empty());
        password_errors.set(errors);
    };

    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldGroup>
                    <Field>
                        <FieldLabel>"Username"</FieldLabel>
                        <Show when=move || username_invalid.get()>
                            <FieldError>"Username is required"</FieldError>
                        </Show>
                        <Input invalid=username_invalid on:input=username_validate />
                    </Field>
                    <Field>
                        <FieldLabel>"Password"</FieldLabel>
                        <FieldError errors=password_errors />
                        <Input
                            input_type="password"
                            invalid=password_invalid
                            on:input=password_validate
                        />
                    </Field>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
