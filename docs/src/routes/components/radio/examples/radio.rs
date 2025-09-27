use leptos::prelude::*;
use singlestage::{radio::*, Label};

#[component]
pub fn RadioExample() -> impl IntoView {
    view! {
        <RadioGroup default="default">
            <Label>
                <Radio value="default" />
                "Default"
            </Label>

            // Radios can create their own labels identical to the code above if provided children.
            <Radio value="comfortable">"Comfortable"</Radio>
            <Radio value="compact">"Compact"</Radio>
        </RadioGroup>
    }
}
