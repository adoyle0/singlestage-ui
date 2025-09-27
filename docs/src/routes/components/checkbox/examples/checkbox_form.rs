use leptos::prelude::*;
use singlestage::{checkbox::*, Label};

#[component]
pub fn CheckboxFormExample() -> impl IntoView {
    view! {
        <form class="form flex flex-row items-start gap-3 rounded-md border p-4 shadow-xs">
            <Checkbox id="checkbox-form-1" />
            <div class="flex flex-col gap-1">
                <Label label_for="checkbox-form-1" class="leading-snug">
                    "Use different settings for my mobile devices"
                </Label>
                <p class="text-muted-foreground text-sm leading-snug">
                    "You can manage your mobile notifications in the mobile settings page."
                </p>
            </div>
        </form>
    }
}
