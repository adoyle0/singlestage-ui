use leptos::prelude::*;
use singlestage::kbd::*;

#[component]
pub fn KbdAnatomy() -> impl IntoView {
    view! {
        <KbdGroup>
            <Kbd />
        </KbdGroup>
    }
}
