use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SliderExample() -> impl IntoView {
    view! { <Slider default=75. max=100. step=1. class="mx-auto w-full max-w-xs" /> }
}
