use leptos::prelude::*;
use singlestage::avatar::*;

#[component]
pub fn AvatarFallbackExample() -> impl IntoView {
    view! {
        <Avatar class="size-12">
            <AvatarFallback>"AD"</AvatarFallback>
        </Avatar>
    }
}
