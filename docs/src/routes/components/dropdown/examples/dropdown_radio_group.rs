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
                    <RadioGroup value=position>
                        <RadioItem value="top">"Top"</RadioItem>
                        <RadioItem value="bottom">"Bottom"</RadioItem>
                        <RadioItem value="right">"Right"</RadioItem>
                    </RadioGroup>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
