use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn RadioExample() -> impl IntoView {
    view! {
        <RadioGroup default="comfortable" class="w-fit">
            <Radio value="default">"Default"</Radio>
            <Radio value="comfortable">"Comfortable"</Radio>
            <Radio value="compact">"Compact"</Radio>
        </RadioGroup>
    }
}
