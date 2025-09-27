use leptos::prelude::*;
use singlestage::{Checkbox, Input, Label};

#[component]
pub fn LabelExample() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="grid gap-2">
                <Label label_for="email">"Email"</Label>
                <Input id="email" input_type="email" />
            </div>

            <Label class="grid gap-2">"Password" <Input input_type="password" /></Label>

            <Label>
                <Checkbox />
                "Accept terms and conditions"
            </Label>
        </div>
    }
}
