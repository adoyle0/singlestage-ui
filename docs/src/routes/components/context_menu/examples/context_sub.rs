use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextSubExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <MenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <MenuItem>"Copy" <MenuShortcut>"⌘C"</MenuShortcut></MenuItem>
                    <MenuItem>"Cut" <MenuShortcut>"⌘X"</MenuShortcut></MenuItem>
                </MenuGroup>
                <MenuSub>
                    <MenuSubTrigger>"More Tools"</MenuSubTrigger>
                    <MenuSubContent>
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
            </MenuContent>
        </ContextMenu>
    }
}
