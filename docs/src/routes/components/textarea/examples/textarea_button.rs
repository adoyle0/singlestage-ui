use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaButtonExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-xs gap-2">
            <Textarea placeholder="Type your message here." />
            <Button>"Send message"</Button>
        </div>
    }
}
