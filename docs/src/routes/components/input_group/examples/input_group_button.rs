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
                <Input placeholder="https://x.com/shadcn" readonly=true />
                <InputGroupAddon align="inline-end">
                    <Button
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
                    </Button>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup class="[--radius:9999px]">
                <InputGroupAddon>
                    <Button variant="secondary" size="icon-xs">
                        {icon!(icondata::LuInfo)}
                    </Button>
                </InputGroupAddon>
                <InputGroupAddon class="text-muted-foreground pl-1.5">"https://"</InputGroupAddon>
                <Input id="input-secure-19" />
                <InputGroupAddon align="inline-end">
                    <Button on:click=move |_| is_favorite.set(!is_favorite.get()) size="icon-xs">
                        <Show
                            when=move || is_favorite.get()
                            fallback=move || { { icon!(icondata::LuStar) } }
                        >
                            {icon!(icondata::LuStar, class="fill-blue-600 stroke-blue-600")}
                        </Show>
                    </Button>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Input placeholder="Type to search..." />
                <InputGroupAddon align="inline-end">
                    <Button variant="secondary">"Search"</Button>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
