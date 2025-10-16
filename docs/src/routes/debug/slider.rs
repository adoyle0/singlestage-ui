use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugSlider() -> impl IntoView {
    let disabled = RwSignal::new(true);
    let max = RwSignal::new(100.);
    let min = RwSignal::new(0.);
    let step = RwSignal::new(1.);
    let value = RwSignal::new(42.);

    view! {
        <h1 class="text-4xl font-semibold">"Slider"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Disabled state should not flicker on load or reload"</li>
            <li>
                "The slider highlight should never lose sync with the slider handle or display the
                wrong value"
            </li>
            <li>"The slider handle should never display the wrong value"</li>
            <li>"Value should never go below min"</li>
            <li>"Value should never go above max"</li>
            <li>
                "Setting max, min, and value each to the same value should not trigger an infinite
                effect loop"
            </li>
        </ul>

        <ul class="text-(--muted-foreground)">
            <li>"Disabled: "{move || disabled.get().to_string()}</li>
            <li>"Max: "{move || max.get().to_string()}</li>
            <li>"Min: "{move || min.get().to_string()}</li>
            <li>"Value: "{move || value.get().to_string()}</li>
            <li>"Step: "{move || step.get().to_string()}</li>
        </ul>

        <form id="slider">
            <Slider disabled max min step value />
            <Slider default=42. />
        </form>
        <Label>"Min" <Slider min=0. max=value step value=min /></Label>
        <Label>"Max" <Slider min=value max=100. step value=max /></Label>
        <p class="text-(--muted-foreground)">"Step breaks everything and I have no idea why"</p>
        <Label>"Step" <Slider min=0.1 max=10. step=0.1 value=step /></Label>

        <span class="space-x-2">
            <Button variant="outline" on:click=move |_| { disabled.set(!disabled.get_untracked()) }>
                "Toggle Disabled"
            </Button>
            <Button variant="outline" on:click=move |_| { value.set(10.) }>
                "10"
            </Button>
            <Button variant="outline" on:click=move |_| { value.set(50.) }>
                "50"
            </Button>
            <Button variant="outline" on:click=move |_| { value.set(70.) }>
                "70"
            </Button>
        </span>

        <div>
            <p class="text-(--muted-foreground)">
                "I can't access onreset and I have no idea why. Maybe it's related to step?
                Anyway, the slider is mostly useful so it's getting released."
            </p>
            <Button button_type="reset" form="slider">
                "Reset"
            </Button>
        </div>
    }
}
