use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ProgressControlledExample() -> impl IntoView {
    let value = RwSignal::new(0.);

    view! {
        <div class="flex w-full max-w-sm flex-col gap-4">
            <Progress value />
            <Slider value />
        </div>
    }
}
