use leptos::prelude::*;
use singlestage::*;
use std::time::Duration;

#[component]
pub fn InputGroupButtonExample() -> impl IntoView {
    let is_copied = RwSignal::new(false);
    let is_favorite = RwSignal::new(false);

    Effect::new(move || {
        if is_copied.get() {
            set_timeout(move || is_copied.set(false), Duration::from_secs(2));
        }
    });

    view! {
        <div class="grid w-full max-w-sm gap-6">
            <InputGroup>
                <InputGroupInput placeholder="https://x.com/shadcn" readonly=true />
                <InputGroupAddon align="inline-end">
                    <InputGroupButton
                        // aria_label="Copy"
                        // title="Copy"
                        size="icon-xs"
                        on:click=move |_| is_copied.set(true)
                    >
                        <Show
                            when=move || is_copied.get()
                            fallback=move || { icon!(icondata::LuCopy) }
                        >
                            {icon!(icondata::LuCheck)}
                        </Show>
                    </InputGroupButton>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup class="[--radius:9999px]">
                <InputGroupAddon>
                    <InputGroupButton variant="secondary" size="icon-xs">
                        {icon!(icondata::LuInfo)}
                    </InputGroupButton>
                </InputGroupAddon>
                <InputGroupAddon class="text-muted-foreground pl-1.5">"https://"</InputGroupAddon>
                <InputGroupInput id="input-secure-19" />
                <InputGroupAddon align="inline-end">
                    <InputGroupButton
                        on:click=move |_| is_favorite.set(!is_favorite.get())
                        size="icon-xs"
                    >
                        <Show
                            when=move || is_favorite.get()
                            fallback=move || { { icon!(icondata::LuStar) } }
                        >
                            {icon!(icondata::LuStar, class="fill-blue-600 stroke-blue-600")}
                        </Show>
                    </InputGroupButton>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupInput placeholder="Type to search..." />
                <InputGroupAddon align="inline-end">
                    <InputGroupButton variant="secondary">"Search"</InputGroupButton>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
