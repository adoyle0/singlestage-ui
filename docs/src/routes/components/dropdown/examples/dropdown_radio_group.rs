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
                        <Radio value="top">"Top"</Radio>
                        <Radio value="bottom">"Bottom"</Radio>
                        <Radio value="right">"Right"</Radio>
                    </RadioGroup>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
