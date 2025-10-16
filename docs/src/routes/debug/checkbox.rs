use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugCheckbox() -> impl IntoView {
    let checked = RwSignal::new(true);
    let disabled = RwSignal::new(true);

    view! {
        <h1 class="text-4xl font-semibold">"Checkbox"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Checked state should not flicker on load or reload"</li>
            <li>
                "Checked state should not lose sync with the signal when alternating clicking the
                button and the checkbox"
            </li>
            <li>"Disabled state should not flicker on load or reload"</li>
        </ul>

        <ul class="text-(--muted-foreground)">
            <li>"Checked: "{move || checked.get().to_string()}</li>
            <li>"Disabled: "{move || disabled.get().to_string()}</li>
        </ul>

        <Label>
            <Checkbox disabled checked />
            "Accept terms and conditions"
        </Label>

        <span class="space-x-2">
            <Button variant="outline" on:click=move |_| { disabled.set(!disabled.get_untracked()) }>
                "Toggle Disabled"
            </Button>
            <Button variant="outline" on:click=move |_| { checked.set(!checked.get_untracked()) }>
                "Toggle Checked"
            </Button>
        </span>
    }
}
