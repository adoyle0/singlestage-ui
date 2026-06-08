use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarBadgeExample() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
            <AvatarFallback>"CN"</AvatarFallback>
            <AvatarBadge class="bg-green-600 dark:bg-green-800" />
        </Avatar>
    }
}
