use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarGroupExample() -> impl IntoView {
    view! {
        <AvatarGroup class="grayscale">
            <Avatar>
                <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
                <AvatarFallback>"CN"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage src="https://github.com/maxleiter.png" alt="@maxleiter" />
                <AvatarFallback>"LR"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage src="https://github.com/evilrabbit.png" alt="@evilrabbit" />
                <AvatarFallback>"ER"</AvatarFallback>
            </Avatar>
        </AvatarGroup>
    }
}
