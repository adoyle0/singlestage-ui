use leptos::prelude::*;
use singlestage::avatar::*;

#[component]
pub fn AvatarExample() -> impl IntoView {
    view! {
        <Avatar class="size-12">
            <AvatarImage alt="@adoyle0" src="/avatar.jpg" />
            <AvatarFallback>"AD"</AvatarFallback>
        </Avatar>
    }
}
