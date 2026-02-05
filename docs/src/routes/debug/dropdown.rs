use leptos::prelude::*;
use singlestage::*;

#[component]
fn Item() -> impl IntoView {
    view! { <MenuItem on:click=move |_| { leptos::logging::log!("hey") }>"Click me"</MenuItem> }
}

#[component]
pub fn DebugDropdown() -> impl IntoView {
    let open = RwSignal::new(false);

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

        // top

        <DropdownMenu>
            <Trigger>
                <Button>"Top Start"</Button>
            </Trigger>
            <MenuContent side="top">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button>"Top Center"</Button>
            </Trigger>
            <MenuContent align="center" side="top">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button>"Top End"</Button>
            </Trigger>
            <MenuContent align="end" side="top">
                <Item />
            </MenuContent>
        </DropdownMenu>

        // right

        <DropdownMenu>
            <Trigger>
                <Button>"Right Start"</Button>
            </Trigger>
            <MenuContent side="right">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button>"Right Center"</Button>
            </Trigger>
            <MenuContent align="center" side="right">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button>"Right End"</Button>
            </Trigger>
            <MenuContent align="end" side="right">
                <Item />
            </MenuContent>
        </DropdownMenu>

        // bottom

        <DropdownMenu>
            <Trigger>
                <Button>"Bottom Start"</Button>
            </Trigger>
            <MenuContent>
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button size="sm">"Bottom Center"</Button>
            </Trigger>
            <MenuContent align="center">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button>"Bottom End"</Button>
            </Trigger>
            <MenuContent align="end">
                <Item />
            </MenuContent>
        </DropdownMenu>

        // left

        <DropdownMenu>
            <Trigger>
                <Button>"Left Start"</Button>
            </Trigger>
            <MenuContent side="left">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button>"Left Center"</Button>
            </Trigger>
            <MenuContent align="center" side="left">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <Trigger>
                <Button>"Left End"</Button>
            </Trigger>
            <MenuContent align="end" side="left">
                <Item />
            </MenuContent>
        </DropdownMenu>
    }
}
