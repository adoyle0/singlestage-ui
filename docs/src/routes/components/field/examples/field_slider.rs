use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSliderExample() -> impl IntoView {
    let value = RwSignal::new(0.);

    view! {
        <Field class="w-full max-w-xs">
            <FieldTitle>"Price Range"</FieldTitle>
            <FieldDescription>
                "Set your budget range ($" <span class="font-medium tabular-nums">"0"</span> " - $"
                <span class="font-medium tabular-nums">{move || value.get()}</span>")."
            </FieldDescription>
            <Slider class="mt-2 w-full" max=1000. min=0. step=10. value />
        </Field>
    }
}
