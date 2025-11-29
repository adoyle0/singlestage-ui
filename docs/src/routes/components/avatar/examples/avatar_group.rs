use leptos::prelude::*;
use singlestage::avatar::*;

#[component]
pub fn AvatarGroupExample() -> impl IntoView {
    view! {
        <AvatarGroup>
            <Avatar class="size-12 grayscale">
                <AvatarImage src="/avatar-1.png" alt="Sofia Davis's Avatar" />
                <AvatarFallback>"SD"</AvatarFallback>
            </Avatar>
            <Avatar class="size-12 grayscale">
                <AvatarImage src="/avatar-2.png" alt="Jackson Lee's Avatar" />
                <AvatarFallback>"JL"</AvatarFallback>
            </Avatar>
            <Avatar class="size-12 grayscale">
                <AvatarImage src="/avatar-3.png" alt="Isabella Nguyen's Avatar" />
                <AvatarFallback>"IN"</AvatarFallback>
            </Avatar>
        </AvatarGroup>
    }
}
