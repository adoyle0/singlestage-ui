use leptos::prelude::*;
use singlestage::badge::*;

#[component]
pub fn BadgeInvalidExample() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Badge attr:aria-invalid="true">"Primary"</Badge>
            <Badge attr:aria-invalid="true" variant="secondary">
                "Secondary"
            </Badge>
            <Badge attr:aria-invalid="true" variant="destructive">
                "Destructive"
            </Badge>
            <Badge attr:aria-invalid="true" variant="outline">
                "Outline"
            </Badge>
        </div>
    }
}
