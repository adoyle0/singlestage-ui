use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarBasicExample() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" class="grayscale" />
            <AvatarFallback>"CN"</AvatarFallback>
        </Avatar>
    }
}
