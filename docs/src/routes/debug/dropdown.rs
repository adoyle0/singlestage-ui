use leptos::prelude::*;
use singlestage::*;

#[component]
fn Item() -> impl IntoView {
    view! { <MenuItem on:click=move |_| { leptos::logging::log!("hey") }>"Click me"</MenuItem> }
}

#[component]
pub fn DropdownTriggersDialog() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Dialog open>
            <DialogContent>"hey"</DialogContent>

            <DropdownMenu>
                <DropdownMenuTrigger>
                    <Button>"Dialog Menu"</Button>
                </DropdownMenuTrigger>
                <MenuContent>
                    <MenuItem>"Close dropdown"</MenuItem>
                    <MenuItem dismiss=false>"Don't dismiss"</MenuItem>
                    <Trigger>
                        <MenuItem>"Implicit trigger"</MenuItem>
                    </Trigger>
                    <Trigger>
                        <MenuItem>"Implicit no dismiss"</MenuItem>
                    </Trigger>
                    <MenuItem on:click={move |ev| {
                        ev.prevent_default();
                        open.set(true);
                    }}>"Open with signal"</MenuItem>
                    <MenuItem
                        dismiss=false
                        on:click={move |ev| {
                            ev.prevent_default();
                            open.set(true);
                        }}
                    >
                        "Open with signal no dismiss"
                    </MenuItem>
                    <MenuItem as_child=true>
                        <Button on:click={move |ev| {
                            ev.prevent_default();
                            open.set(true);
                        }}>"Child open with signal"</Button>
                    </MenuItem>
                    <MenuItem dismiss=false as_child=true>
                        <Button on:click={move |ev| {
                            ev.prevent_default();
                            open.set(true);
                        }}>"Child open with signal no dismiss"</Button>
                    </MenuItem>
                </MenuContent>
            </DropdownMenu>
        </Dialog>
    }
}

#[component]
pub fn DebugDropdown() -> impl IntoView {
    let open = RwSignal::new(false);

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
        <h1 class="text-4xl font-semibold">"Dropdown Menu"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Console should log \"hey\" when clicked"</li>
            <li>"Console should log \"hey\" when tapped"</li>
            <li>
                "Console should log \"hey\" when selected with keyboard and "<Kbd>"Enter"</Kbd>
                " is pressed"
            </li>
        </ul>

        <Switch checked=open />

        <DropdownMenu open dismissable=false>
            <Trigger>
                <Button>"Signal"</Button>
            </Trigger>
            <MenuContent>
                <Item />
                <MenuItem dismiss=false>"No dismiss"</MenuItem>
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button>"No dismiss"</Button>
            </Trigger>
            <MenuContent>
                <MenuItem dismiss=false>"no dismiss"</MenuItem>
            </MenuContent>
        </DropdownMenu>

        <p>"Positioning"</p>
        <For each=move || variations key=|_| "42" let(variation)>
            <DropdownMenu>
                <Trigger>
                    <Button size="xs">{variation.0}" "{variation.1}</Button>
                </Trigger>
                <MenuContent side={variation.0} align={variation.1}>
                    <Item />
                </MenuContent>
            </DropdownMenu>
        </For>

        <p>"Dialog Trigger"</p>
        <DropdownTriggersDialog />
    }
}
