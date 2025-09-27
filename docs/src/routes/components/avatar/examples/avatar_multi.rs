use leptos::prelude::*;
use singlestage::avatar::*;

#[component]
pub fn AvatarMultiExample() -> impl IntoView {
    view! {
        <div class="flex -space-x-2">
            <Avatar>
                <AvatarFallback>"AB"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarFallback>"CD"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarFallback>"EF"</AvatarFallback>
            </Avatar>
        </div>
    }
}
