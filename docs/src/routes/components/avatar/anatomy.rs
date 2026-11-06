use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarAnatomy() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage />
            <AvatarFallback />
            <AvatarBadge />
        </Avatar>

        // Group
        <AvatarGroup>
            <Avatar />
            <Avatar />
            <Avatar />
            <AvatarGroupCount />
        </AvatarGroup>
    }
}
