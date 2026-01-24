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
            <DropdownMenuTrigger>
                <Button>"Signal"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        // top

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Top Start"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent side="top">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Top Center"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="center" side="top">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Top End"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" side="top">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        // right

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Right Start"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent side="right">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Right Center"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="center" side="right">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Right End"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" side="right">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        // bottom

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Bottom Start"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button size="sm">"Bottom Center"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="center">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Bottom End"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        // left

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Left Start"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent side="left">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Left Center"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="center" side="left">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>

        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button>"Left End"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" side="left">
                <Item />
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
