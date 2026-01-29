use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextRadioExample() -> impl IntoView {
    let user = RwSignal::new("pedro".to_string());
    let theme = RwSignal::new("light".to_string());

    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuGroup>
                    <ContextMenuLabel>"People"</ContextMenuLabel>
                    <RadioGroup value=user>
                        <RadioItem value="pedro">"Pedro Duarte"</RadioItem>
                        <RadioItem value="colm">"Colm Tuite"</RadioItem>
                    </RadioGroup>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuLabel>"Theme"</ContextMenuLabel>
                    <RadioGroup value=theme>
                        <RadioItem value="light">"Light"</RadioItem>
                        <RadioItem value="dark">"Dark"</RadioItem>
                        <RadioItem value="system">"System"</RadioItem>
                    </RadioGroup>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
