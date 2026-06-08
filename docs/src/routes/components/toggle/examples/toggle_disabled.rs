use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleDisabledExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-2">
            <Toggle aria_label="Toggle disabled" disabled=true>
                "Disabled"
            </Toggle>
            <Toggle variant="outline" aria_label="Toggle disabled outline" disabled=true>
                "Disabled"
            </Toggle>
        </div>
    }
}
