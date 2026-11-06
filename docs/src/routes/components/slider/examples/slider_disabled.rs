use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SliderDisabledExample() -> impl IntoView {
    view! { <Slider default=50. max=100. step=1. disabled=true class="mx-auto w-full max-w-xs" /> }
}
