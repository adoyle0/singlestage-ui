use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownRadioGroupExample() -> impl IntoView {
    let position = RwSignal::new("bottom".to_string());

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Open"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-32">
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"Panel Position"</DropdownMenuLabel>
                    <DropdownMenuRadioGroup value=position>
                        <DropdownMenuRadioItem value="top">"Top"</DropdownMenuRadioItem>
                        <DropdownMenuRadioItem value="bottom">"Bottom"</DropdownMenuRadioItem>
                        <DropdownMenuRadioItem value="right">"Right"</DropdownMenuRadioItem>
                    </DropdownMenuRadioGroup>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
