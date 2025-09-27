use leptos::prelude::*;
use singlestage::avatar::*;

#[component]
pub fn AvatarAnatomy() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage />
            <AvatarFallback />
        </Avatar>
    }
}
