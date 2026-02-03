use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownRadioGroupExample() -> impl IntoView {
    let position = RwSignal::new("bottom".to_string());

    view! {
        <DropdownMenu>
            <MenuTrigger>
                <Button variant="outline">"Open"</Button>
            </MenuTrigger>
            <MenuContent class="w-32">
                <MenuGroup>
                    <Label>"Panel Position"</Label>
                    <RadioGroup value=position>
                        <RadioItem value="top">"Top"</RadioItem>
                        <RadioItem value="bottom">"Bottom"</RadioItem>
                        <RadioItem value="right">"Right"</RadioItem>
                    </RadioGroup>
                </MenuGroup>
            </MenuContent>
        </DropdownMenu>
    }
}
