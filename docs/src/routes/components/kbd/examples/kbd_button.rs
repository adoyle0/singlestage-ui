use leptos::prelude::*;
use singlestage::{Button, Kbd};

#[component]
pub fn KbdButtonExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-4">
            <Button variant="outline" size="small" class="pr-2">
                "Accept"
                <Kbd>"⏎"</Kbd>
            </Button>
            <Button variant="outline" size="small" class="pr-2">
                "Cancel"
                <Kbd>"Esc"</Kbd>
            </Button>
        </div>
    }
}
