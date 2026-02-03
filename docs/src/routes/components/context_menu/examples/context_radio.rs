use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextRadioExample() -> impl IntoView {
    let user = RwSignal::new("pedro".to_string());
    let theme = RwSignal::new("light".to_string());

    view! {
        <ContextMenu>
            <MenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <Label>"People"</Label>
                    <RadioGroup value=user>
                        <RadioItem value="pedro">"Pedro Duarte"</RadioItem>
                        <RadioItem value="colm">"Colm Tuite"</RadioItem>
                    </RadioGroup>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <Label>"Theme"</Label>
                    <RadioGroup value=theme>
                        <RadioItem value="light">"Light"</RadioItem>
                        <RadioItem value="dark">"Dark"</RadioItem>
                        <RadioItem value="system">"System"</RadioItem>
                    </RadioGroup>
                </MenuGroup>
            </MenuContent>
        </ContextMenu>
    }
}
