use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerSizeExample() -> impl IntoView {
    view! {
        <div class="flex items-center gap-6">
            <Spinner class="size-3" />
            <Spinner class="size-4" />
            <Spinner class="size-6" />
            <Spinner class="size-8" />
        </div>
    }
}
