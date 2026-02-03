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
            <MenuTrigger>
                <Button>"Signal"</Button>
            </MenuTrigger>
            <MenuContent>
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button>"No dismiss"</Button>
            </MenuTrigger>
            <MenuContent>
                <MenuItem dismiss=false>"no dismiss"</MenuItem>
            </MenuContent>
        </DropdownMenu>

        // top

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Top Start"</Button>
            </MenuTrigger>
            <MenuContent side="top">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Top Center"</Button>
            </MenuTrigger>
            <MenuContent align="center" side="top">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Top End"</Button>
            </MenuTrigger>
            <MenuContent align="end" side="top">
                <Item />
            </MenuContent>
        </DropdownMenu>

        // right

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Right Start"</Button>
            </MenuTrigger>
            <MenuContent side="right">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Right Center"</Button>
            </MenuTrigger>
            <MenuContent align="center" side="right">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Right End"</Button>
            </MenuTrigger>
            <MenuContent align="end" side="right">
                <Item />
            </MenuContent>
        </DropdownMenu>

        // bottom

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Bottom Start"</Button>
            </MenuTrigger>
            <MenuContent>
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button size="sm">"Bottom Center"</Button>
            </MenuTrigger>
            <MenuContent align="center">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Bottom End"</Button>
            </MenuTrigger>
            <MenuContent align="end">
                <Item />
            </MenuContent>
        </DropdownMenu>

        // left

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Left Start"</Button>
            </MenuTrigger>
            <MenuContent side="left">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Left Center"</Button>
            </MenuTrigger>
            <MenuContent align="center" side="left">
                <Item />
            </MenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <MenuTrigger>
                <Button>"Left End"</Button>
            </MenuTrigger>
            <MenuContent align="end" side="left">
                <Item />
            </MenuContent>
        </DropdownMenu>
    }
}
