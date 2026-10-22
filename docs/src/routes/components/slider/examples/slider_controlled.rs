use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SliderControlledExample() -> impl IntoView {
    let value = RwSignal::new(0.3);

    view! {
        <div class="mx-auto grid w-full max-w-xs gap-3">
            <div class="flex items-center justify-between gap-2">
                <Label label_for="slider-demo-temperature">"Temperature"</Label>
                <span class="text-sm text-muted-foreground">{move || value.get().to_string()}</span>
            </div>
            <Slider id="slider-demo-temperature" value min=0. max=1. step=0.1 />
        </div>
    }
}
