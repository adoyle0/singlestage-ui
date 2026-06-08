use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BadgeExample() -> impl IntoView {
    view! {
        <div class="flex w-full flex-wrap justify-center gap-2">
            <Badge>"Badge"</Badge>
            <Badge variant="secondary">"Secondary"</Badge>
            <Badge variant="destructive">"Destructive"</Badge>
            <Badge variant="outline">"Outline"</Badge>
        </div>
    }
}
