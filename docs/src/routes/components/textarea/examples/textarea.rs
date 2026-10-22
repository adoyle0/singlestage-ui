use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaExample() -> impl IntoView {
    view! {
        <div class="w-xs">
            <Textarea placeholder="Type your message here" />
        </div>
    }
}
