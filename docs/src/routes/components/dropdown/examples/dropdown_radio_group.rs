use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownRadioGroupExample() -> impl IntoView {
    let position = RwSignal::new("bottom".to_string());

    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="outline">"Open"</Button>
            </Trigger>
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
