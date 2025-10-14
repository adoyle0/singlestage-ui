use leptos::prelude::*;
use singlestage::{Label, checkbox::*};

#[component]
pub fn SubtextExample() -> impl IntoView {
    view! {
        <div class="flex items-start gap-3">
            <Checkbox id="checkbox-with-text" />
            <div class="grid gap-2">
                <Label label_for="checkbox-with-text">"Accept terms and conditions"</Label>
                <p class="text-(--muted-foreground) text-sm">
                    "By clicking this checkbox, you agree to the terms and conditions."
                </p>
            </div>
        </div>
    }
}
