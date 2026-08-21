use leptos::prelude::*;
use singlestage::*;

#[component]
fn Item() -> impl IntoView {
    view! {
        <DropdownMenuItem on:click=move |_| {
            leptos::logging::log!("hey")
        }>"Click me"</DropdownMenuItem>
    }
}

#[component]
pub fn DropdownTriggersDialog() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <p>"Dialog Trigger"</p>

        <Dialog open>
            <DialogContent>"hey"</DialogContent>

            <DropdownMenu>
                <DropdownMenuTrigger>
                    <Button>"Dialog Menu"</Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    <DropdownMenuItem>"Close dropdown"</DropdownMenuItem>
                    <DropdownMenuItem dismiss=false>"Don't dismiss"</DropdownMenuItem>
                    <DialogTrigger>
                        <DropdownMenuItem>"Implicit trigger"</DropdownMenuItem>
                    </DialogTrigger>
                    <DialogTrigger>
                        <DropdownMenuItem>"Implicit no dismiss"</DropdownMenuItem>
                    </DialogTrigger>
                    <DropdownMenuItem on:click={move |ev| {
                        ev.prevent_default();
                        open.set(true);
                    }}>"Open with signal"</DropdownMenuItem>
                    <DropdownMenuItem
                        dismiss=false
                        on:click={move |ev| {
                            ev.prevent_default();
                            open.set(true);
                        }}
                    >
                        "Open with signal no dismiss"
                    </DropdownMenuItem>
                    <DropdownMenuItem as_child=true>
                        <Button on:click={move |ev| {
                            ev.prevent_default();
                            open.set(true);
                        }}>"Child open with signal"</Button>
                    </DropdownMenuItem>
                    <DropdownMenuItem dismiss=false as_child=true>
                        <Button on:click={move |ev| {
                            ev.prevent_default();
                            open.set(true);
                        }}>"Child open with signal no dismiss"</Button>
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </Dialog>
    }
}

#[component]
pub fn DropdownSignal() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Switch checked=open />

        <DropdownMenu open modal=true>
            <DropdownMenuTrigger>
                <Button>"Signal"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <Item />
                <DropdownMenuItem dismiss=false>"No dismiss"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}

#[component]
pub fn DropdownDismiss() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"No dismiss"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuItem dismiss=false>"no dismiss"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}

#[component]
pub fn DropdownPosition() -> impl IntoView {
    let variations = [
        ("top", "start"),
        ("top", "center"),
        ("top", "end"),
        ("right", "start"),
        ("right", "center"),
        ("right", "end"),
        ("bottom", "start"),
        ("bottom", "center"),
        ("bottom", "end"),
        ("left", "start"),
        ("left", "center"),
        ("left", "end"),
    ];

    view! {
        <p>"Positioning"</p>
        <For each=move || variations key=|_| "42" let(variation)>
            <DropdownMenu>
                <DropdownMenuTrigger>
                    <Button size="xs">{variation.0}" "{variation.1}</Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent side={variation.0} align={variation.1}>
                    <Item />
                </DropdownMenuContent>
            </DropdownMenu>
        </For>
    }
}

#[component]
pub fn DebugDropdown() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Dropdown Menu"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Console should log \"hey\" when clicked"</li>
            <li>"Console should log \"hey\" when tapped"</li>
            <li>
                "Console should log \"hey\" when selected with keyboard and "<Kbd>"Enter"</Kbd>
                " is pressed"
            </li>
        </ul>

        <DropdownSignal />
        <DropdownDismiss />
        <DropdownPosition />

        <DropdownTriggersDialog />
    }
}
