use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSliderExample() -> impl IntoView {
    let value = RwSignal::new(0.);

    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldGroup>
                    <Field>
                        <FieldLabel>"Price Range"</FieldLabel>
                        <FieldDescription>
                            "Set your budget range ($"
                            <span class="font-medium tabular-nums">"0"</span> " - $"
                            <span class="font-medium tabular-nums">{move || value.get()}</span>")."
                        </FieldDescription>
                        <Slider class="mt-2 w-full" max=1000. min=0. step=10. value />
                    </Field>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
