use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleSizeExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-2">
            <Toggle variant="outline" aria_label="Toggle small" size="sm">
                "Small"
            </Toggle>
            <Toggle variant="outline" aria_label="Toggle default" size="default">
                "Default"
            </Toggle>
            <Toggle variant="outline" aria_label="Toggle large" size="lg">
                "Large"
            </Toggle>
        </div>
    }
}
