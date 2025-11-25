use leptos::prelude::*;
use singlestage::*;
use std::time::Duration;

#[component]
pub fn ProgressExample() -> impl IntoView {
    let value = RwSignal::new(13);

    Effect::new(move || {
        set_timeout(move || value.set(66), Duration::from_millis(500));
    });

    view! { <Progress class="w-[60%]" value /> }
}
