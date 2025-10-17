use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleOutlineExample() -> impl IntoView {
    view! {
                <Toggle
                    variant="outline"
                    pressed=RwSignal::new(false)
                    title="Toggle italic text"
                    attr:aria-label="Toggle italic text"
                >
                    <span class="font-semibold leading-none">"B"</span>
                </Toggle>
    }
}
