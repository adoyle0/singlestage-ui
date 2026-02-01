use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BadgeVariantsExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <Badge invalid=true>"Invalid"</Badge>
            <Badge variant="secondary">"Secondary"</Badge>
            <Badge variant="destructive">"Destructive"</Badge>
            <Badge variant="outline">"Outline"</Badge>
            <Badge variant="ghost">"Ghost"</Badge>
        </div>
    }
}
