use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BadgeExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <Badge>"Badge"</Badge>
            <Badge variant="secondary">"Secondary"</Badge>
            <Badge variant="destructive">"Destructive"</Badge>
            <Badge variant="outline">"Outline"</Badge>
        </div>
    }
}
