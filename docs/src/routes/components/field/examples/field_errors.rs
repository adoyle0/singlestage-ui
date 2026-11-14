use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldErrorsExample() -> impl IntoView {
    let username_invalid = RwSignal::new(false);

    let validate_username = move |ev| {
        username_invalid.set(event_target_value(&ev).is_empty());
    };

    let password_errors: RwSignal<Vec<String>> = RwSignal::new(vec![]);

    let validate_password = move |ev| {
        let password = event_target_value(&ev);
        let mut errors = vec![];

        if password.is_empty() {
            errors.push("Password is required".to_string());
        };

        if password.len() < 8 {
            errors.push("Password must be at least 8 characters long".to_string());
        };

        if !password.chars().any(|char| char.is_numeric()) {
            errors.push("Password must contain a number".to_string());
        };

        if !password.chars().any(|char| char.is_uppercase()) {
            errors.push("Password must contain uppercase letters".to_string());
        };

        if !password.chars().any(|char| char.is_lowercase()) {
            errors.push("Password must contain lowercase letters".to_string());
        };

        password_errors.set(errors);
    };

    // Set errors for demonstration purposes
    username_invalid.set(true);
    password_errors.set(vec![
        "Password is required".to_string(),
        "Password must be at least 8 characters long".to_string(),
        "Password must contain a number".to_string(),
        "Password must contain uppercase letters".to_string(),
        "Password must contain lowercase letters".to_string(),
    ]);

    view! {
        <FieldGroup class="max-w-xs">
            // Children
            <Field invalid=username_invalid>
                <Input on:input=validate_username>"Username"</Input>
                <FieldError when=username_invalid>"Username is required"</FieldError>
            </Field>

            // Array
            <Field>
                <Input input_type="password" on:input=validate_password>
                    "Password"
                </Input>
                <FieldError errors=password_errors />
            </Field>
        </FieldGroup>
    }
}
