use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SliderControlledExample() -> impl IntoView {
    let value = RwSignal::new(0.3);

    view! {
        <Field class="w-full max-w-xs">
            <FieldLabel>
                <span class="font-medium">"Temperature"</span>
                <span class="ml-auto text-(--muted-foreground)">
                    {move || value.get().to_string()}
                </span>
            </FieldLabel>
            <Slider value min=0. max=1. step=0.1 />
        </Field>
    }
}
