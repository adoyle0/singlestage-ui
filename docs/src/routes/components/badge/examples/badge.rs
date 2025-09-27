use leptos::prelude::*;
use singlestage::badge::*;

#[component]
pub fn BadgeExample() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Badge>"Primary"</Badge>
            <Badge variant="secondary">"Secondary"</Badge>
            <Badge variant="destructive">"Destructive"</Badge>
            <Badge variant="outline">"Outline"</Badge>
        </div>
    }
}
