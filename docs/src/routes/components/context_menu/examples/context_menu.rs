use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextMenuExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <Trigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </Trigger>
            <MenuContent class="w-48">
                <MenuGroup>
                    <MenuItem>"Back" <MenuShortcut>"⌘["</MenuShortcut></MenuItem>
                    <MenuItem disabled=true>"Forward" <MenuShortcut>"⌘]"</MenuShortcut></MenuItem>
                    <MenuItem>"Reload" <MenuShortcut>"⌘R"</MenuShortcut></MenuItem>
                    <MenuSub>
                        <MenuSubTrigger>"More Tools"</MenuSubTrigger>
                        <MenuSubContent class="w-44">
                            <MenuGroup>
                                <MenuItem>"Save Page..."</MenuItem>
                                <MenuItem>"Create Shortcut..."</MenuItem>
                                <MenuItem>"Name Window..."</MenuItem>
                            </MenuGroup>
                            <Separator />
                            <MenuGroup>
                                <MenuItem>"Developer Tools"</MenuItem>
                            </MenuGroup>
                            <Separator />
                            <MenuGroup>
                                <MenuItem variant="destructive">"Delete"</MenuItem>
                            </MenuGroup>
                        </MenuSubContent>
                    </MenuSub>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <CheckboxItem checked=true>"Show Bookmarks"</CheckboxItem>
                    <CheckboxItem>"Show Full URLs"</CheckboxItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <RadioGroup value="pedro">
                        <Label>"People"</Label>
                        <RadioItem value="pedro">"Pedro Duarte"</RadioItem>
                        <RadioItem value="colm">"Colm Tuite"</RadioItem>
                    </RadioGroup>
                </MenuGroup>
            </MenuContent>
        </ContextMenu>
    }
}
