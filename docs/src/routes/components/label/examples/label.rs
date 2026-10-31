use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn LabelExample() -> impl IntoView {
    view! {
        <div class="flex gap-2">
            <Checkbox id="terms" />
            <Label label_for="terms">"Accept terms and conditions"</Label>
        </div>
    }
}
