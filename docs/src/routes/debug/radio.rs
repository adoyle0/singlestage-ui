use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugRadio() -> impl IntoView {
    let disabled = RwSignal::new(true);
    let invalid = RwSignal::new(true);
    let value = RwSignal::new("two".to_string());

    view! {
        <h1 class="text-4xl font-semibold">"Radio Group"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Selected value should not flicker on load or reload"</li>
            <li>
                "Selected value should not lose sync with the signal when alternating clicking the
                button and the select"
            </li>
            <li>"Disabled state should not flicker on load or reload"</li>
            <li>"Invalid state should not flicker on load or reload"</li>
        </ul>

        <ul class="text-(--muted-foreground)">
            <li>"Disabled: "{move || disabled.get().to_string()}</li>
            <li>"Invalid: "{move || invalid.get().to_string()}</li>
            <li>"Value: "{move || value.get().to_string()}</li>
        </ul>

        <RadioGroup disabled invalid value>
            <Radio value="one">"One"</Radio>
            <Radio value="two">"Two"</Radio>
            <Radio value="three">"Three"</Radio>
        </RadioGroup>

        <span class="space-x-2">
            <Button variant="outline" on:click=move |_| { disabled.set(!disabled.get_untracked()) }>
                "Toggle Disabled"
            </Button>
            <Button variant="outline" on:click=move |_| { invalid.set(!invalid.get_untracked()) }>
                "Toggle Invalid"
            </Button>
            <Button variant="outline" on:click=move |_| { value.set("one".to_string()) }>
                "One"
            </Button>
            <Button variant="outline" on:click=move |_| { value.set("two".to_string()) }>
                "Two"
            </Button>
            <Button variant="outline" on:click=move |_| { value.set("three".to_string()) }>
                "Three"
            </Button>
        </span>
    }
}
