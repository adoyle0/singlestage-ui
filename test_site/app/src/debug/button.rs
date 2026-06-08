use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugButton() -> impl IntoView {
    let disabled = RwSignal::new(true);
    view! {
        <h1 class="text-4xl font-semibold">"Button"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Disabled state should not flicker on load or reload"</li>
            <li>"Disabled state should not lose sync with the signal"</li>
        </ul>

        <ul class="text-(--muted-foreground)">
            <li>"Disabled: "{move || disabled.get().to_string()}</li>
        </ul>

        <Button disabled>"Test button"</Button>

        <Button on:click=move |_| disabled.set(!disabled.get())>"Toggle Disabled"</Button>
    }
}
