use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-xs">
            <Textarea placeholder="Type your message here" />
        </div>
    }
}
