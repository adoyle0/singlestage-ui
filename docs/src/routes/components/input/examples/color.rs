use leptos::prelude::*;
use singlestage::input::*;

#[component]
pub fn ColorExample() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <Input input_type="color" default_value="#F00" />
            <Input input_type="color" default_value="#0F0" invalid=true />
            <Input input_type="color" default_value="#00F" disabled=true />
        </div>
    }
}
