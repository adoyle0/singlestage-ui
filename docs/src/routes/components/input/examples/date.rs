use leptos::prelude::*;
use singlestage::input::*;

#[component]
pub fn DateExample() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <Input input_type="date" />
            <Input input_type="date" invalid=true />
            <Input input_type="date" disabled=true />
        </div>
    }
}
