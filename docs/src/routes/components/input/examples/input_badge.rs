use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputBadgeExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <FieldLabel>
                "Webhook URL " <Badge variant="secondary" class="ml-auto">
                    "Beta"
                </Badge>
            </FieldLabel>
            <Input input_type="url" placeholder="https://api.example.com/webhook" />
        </Field>
    }
}
