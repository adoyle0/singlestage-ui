use leptos::prelude::*;
use singlestage::input::*;

#[component]
pub fn EmailExample() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <Input input_type="email" placeholder="Email" />
            <Input input_type="email" placeholder="Email" invalid=true />
            <Input input_type="email" placeholder="Email" disabled=true />
        </div>
    }
}
