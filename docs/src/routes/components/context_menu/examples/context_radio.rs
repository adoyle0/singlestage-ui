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
                        <Radio value="pedro">"Pedro Duarte"</Radio>
                        <Radio value="colm">"Colm Tuite"</Radio>
                    </RadioGroup>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuLabel>"Theme"</ContextMenuLabel>
                    <RadioGroup value=theme>
                        <Radio value="light">"Light"</Radio>
                        <Radio value="dark">"Dark"</Radio>
                        <Radio value="system">"System"</Radio>
                    </RadioGroup>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
