use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleExample() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);
    let underline = RwSignal::new(false);

    view! {
        <div class="flex flex-col gap-4">
            <div class="flex gap-2">
                <Toggle
                    variant="default"
                    pressed=bold
                    title="Toggle bold text"
                    aria-label="Toggle bold text"
                >
                    <span class="font-semibold leading-none">"B"</span>
                </Toggle>

                <Toggle
                    variant="outline"
                    pressed=italic
                    title="Toggle italic text"
                    aria-label="Toggle italic text"
                >
                    <span class="italic leading-none">"I"</span>
                </Toggle>

                <Toggle
                    variant="default"
                    pressed=underline
                    title="disabled"
                    aria-label="disabled"
                    disabled=true
                >
                    <span class="underline leading-none">"U"</span>
                </Toggle>
            </div>

            <p class="text-sm text-muted-foreground">
                {move || {
                    let mut active = Vec::new();

                    if bold.get() {
                        active.push("bold");
                    }

                    if italic.get() {
                        active.push("italic");
                    }

                    if underline.get() {
                        active.push("underline");
                    }

                    if active.is_empty() {
                        "Formatting disabled".to_string()
                    } else {
                        format!("Formatting: {}", active.join(", "))
                    }
                }}
            </p>
        </div>
    }
}
