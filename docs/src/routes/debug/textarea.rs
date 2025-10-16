use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugTextarea() -> impl IntoView {
    let disabled = RwSignal::new(true);
    let invalid = RwSignal::new(true);

    view! {
        <h1 class="text-4xl font-semibold">"Textarea"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Disabled state should not flicker on load or reload"</li>
            <li>"Invalid state should not flicker on load or reload"</li>
        </ul>

        <ul class="text-(--muted-foreground)">
            <li>"Disabled: "{move || disabled.get().to_string()}</li>
            <li>"Invalid: "{move || invalid.get().to_string()}</li>
        </ul>

        <Textarea disabled invalid />

        <span class="space-x-2">
            <Button variant="outline" on:click=move |_| disabled.set(!disabled.get())>
                "Toggle Disabled"
            </Button>
            <Button variant="outline" on:click=move |_| invalid.set(!invalid.get())>
                "Toggle Invalid"
            </Button>
        </span>
    }
}
