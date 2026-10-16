use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarBadgeWithIconExample() -> impl IntoView {
    view! {
        <Avatar class="grayscale">
            <AvatarImage src="https://github.com/pranathip.png" alt="@pranathip" />
            <AvatarFallback>"PP"</AvatarFallback>
            <AvatarBadge>{icon!(icondata::LuPlus)}</AvatarBadge>
        </Avatar>
    }
}
