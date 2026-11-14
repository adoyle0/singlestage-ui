use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SliderVerticalExample() -> impl IntoView {
    view! {
        <div class="mx-auto flex w-full max-w-xs items-center justify-center gap-6">
            <Slider default=50. max=100. step=1. orientation="vertical" class="h-40" />
            <Slider default=25. max=100. step=1. orientation="vertical" class="h-40" />
        </div>
    }
}
