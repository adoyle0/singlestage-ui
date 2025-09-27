use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaExample() -> impl IntoView {
    view! { <Textarea placeholder="Type your message here"></Textarea> }
}
