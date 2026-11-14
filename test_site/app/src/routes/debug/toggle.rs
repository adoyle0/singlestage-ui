use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugToggle() -> impl IntoView {
    let disabled = RwSignal::new(true);
    let pressed = RwSignal::new(true);
    view! {
        <h1 class="text-4xl font-semibold">"Toggle"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Disabled state should not flicker on load or reload"</li>
            <li>"Disabled state should not lose sync with the signal"</li>
            <li>"Pressed state should not flicker on load or reload"</li>
            <li>"Pressed state should not lose sync with the signal"</li>
        </ul>

        <ul class="text-(--muted-foreground)">
            <li>"Disabled: "{move || disabled.get().to_string()}</li>
            <li>"Pressed: "{move || pressed.get().to_string()}</li>
        </ul>

        <Toggle disabled pressed>
            {icon!(icondata::LuBold)}
        </Toggle>

        <Button on:click=move |_| disabled.set(!disabled.get())>"Toggle Disabled"</Button>
        <Button on:click=move |_| pressed.set(!pressed.get())>"Toggle Pressed"</Button>
    }
}
