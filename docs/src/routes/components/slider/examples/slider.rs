use leptos::prelude::*;
use singlestage::{Button, Checkbox, Label, Slider};

#[component]
pub fn SliderExample() -> impl IntoView {
    let disabled = RwSignal::new(true);
    let max = RwSignal::new(60.);
    let min = RwSignal::new(40.);
    let value = RwSignal::new(50.);

    view! {
        <div>
            <div class="flex justify-between text-(--muted-foreground)">
                <span class="w-4">{move || min.get()}</span>
                <span class="w-fit mx-auto">{move || value.get()}</span>
                <span class="w-4">{move || max.get()}</span>
            </div>

            <div>
                <Slider id="value" class="w-full" disabled min max value />

                <Label class="my-4">
                    <Checkbox checked=disabled />
                    "Disabled"
                </Label>

                <div class="flex justify-between">
                    <Button variant="outline" on:click=move |_| value.set(10.)>
                        "10"
                    </Button>
                    <Button variant="outline" on:click=move |_| value.set(50.)>
                        "50"
                    </Button>
                    <Button variant="outline" on:click=move |_| value.set(70.)>
                        "70"
                    </Button>
                </div>
            </div>

            <div>
                <Slider class="ml-auto" min=0. max=value value=min>
                    "Min:"
                </Slider>
                <Slider class="ml-auto" min=value max=100. value=max>
                    "Max:"
                </Slider>
            </div>
        </div>
    }
}
