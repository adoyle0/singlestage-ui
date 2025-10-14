use leptos::prelude::*;
use singlestage::Textarea;

#[component]
pub fn TextareaExample() -> impl IntoView {
    view! {
        <form class="w-full">
            <Textarea placeholder="Type your message here">"Comments:"</Textarea>
        </form>
    }
}
