use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownComplexExample() -> impl IntoView {
    let theme = RwSignal::new("light".to_string());

    struct Notifications {
        email: RwSignal<bool>,
        push: RwSignal<bool>,
    }

    let notifications = Notifications {
        email: RwSignal::new(true),
        push: RwSignal::new(true),
    };

    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="outline">"Complex Menu"</Button>
            </Trigger>
            <MenuContent class="w-44">
                <MenuGroup>
                    <Label>"File"</Label>
                    <MenuItem>
                        {icon!(icondata::LuFile)} "New File" <MenuShortcut>"⌘N"</MenuShortcut>
                    </MenuItem>
                    <MenuItem>
                        {icon!(icondata::LuFolder)} "New Folder"
                        <MenuShortcut>"⇧⌘N"</MenuShortcut>
                    </MenuItem>
                    <MenuSub>
                        <MenuSubTrigger>
                            {icon!(icondata::LuFolderOpen)} "Open Recent"
                        </MenuSubTrigger>
                        <MenuSubContent>
                            <MenuGroup>
                                <Label>"Recent Projects"</Label>
                                <MenuItem>{icon!(icondata::LuFileCode)} "Project Alpha"</MenuItem>
                                <MenuItem>{icon!(icondata::LuFileCode)} "Project Beta"</MenuItem>
                                <MenuSub>
                                    <MenuSubTrigger>
                                        {icon!(icondata::FiMoreHorizontal)} "More Projects"
                                    </MenuSubTrigger>
                                    <MenuSubContent>
                                        <MenuItem>
                                            {icon!(icondata::LuFileCode)} "Project Gamma"
                                        </MenuItem>
                                        <MenuItem>
                                            {icon!(icondata::LuFileCode)} "Project Delta"
                                        </MenuItem>
                                    </MenuSubContent>
                                </MenuSub>
                            </MenuGroup>
                            <Separator />
                            <MenuGroup>
                                <MenuItem>{icon!(icondata::LuFolderSearch)} "Browse..."</MenuItem>
                            </MenuGroup>
                        </MenuSubContent>
                    </MenuSub>
                    <Separator />
                    <MenuItem>
                        {icon!(icondata::LuSave)} "Save" <MenuShortcut>"⌘S"</MenuShortcut>
                    </MenuItem>
                    <MenuItem>
                        {icon!(icondata::LuDownload)} "Export"
                        <MenuShortcut>"⇧⌘E"</MenuShortcut>
                    </MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <Label>"View"</Label>
                    <CheckboxItem checked=true>
                        {icon!(icondata::LuEye)} "Show Sidebar"
                    </CheckboxItem>
                    <CheckboxItem>{icon!(icondata::FiLayout)} "Show Status Bar"</CheckboxItem>
                    <MenuSub>
                        <MenuSubTrigger>{icon!(icondata::LuPalette)} "Theme"</MenuSubTrigger>
                        <MenuSubContent>
                            <MenuGroup>
                                <Label>"Appearance"</Label>
                                <RadioGroup value=theme>
                                    <RadioItem value="light">
                                        {icon!(icondata::LuSun)} "Light"
                                    </RadioItem>
                                    <RadioItem value="dark">
                                        {icon!(icondata::LuMoon)} "Dark"
                                    </RadioItem>
                                    <RadioItem value="system">
                                        {icon!(icondata::LuMonitor)} "System"
                                    </RadioItem>
                                </RadioGroup>
                            </MenuGroup>
                        </MenuSubContent>
                    </MenuSub>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <Label>"Account"</Label>
                    <MenuItem>
                        {icon!(icondata::LuUser)} "Profile" <MenuShortcut>"⇧⌘P"</MenuShortcut>
                    </MenuItem>
                    <MenuItem>{icon!(icondata::LuCreditCard)} "Billing"</MenuItem>
                    <MenuSub>
                        <MenuSubTrigger>{icon!(icondata::LuSettings)} "Settings"</MenuSubTrigger>
                        <MenuSubContent>
                            <MenuGroup>
                                <Label>"Preferences"</Label>
                                <MenuItem>
                                    {icon!(icondata::LuKeyboard)} "Keyboard Shortcuts"
                                </MenuItem>
                                <MenuItem>{icon!(icondata::LuLanguages)} "Language"</MenuItem>
                                <MenuSub>
                                    <MenuSubTrigger>
                                        {icon!(icondata::LuBell)} "Notifications"
                                    </MenuSubTrigger>
                                    <MenuSubContent>
                                        <MenuGroup>
                                            <Label>"Notification Types"</Label>
                                            <CheckboxItem checked=notifications
                                                .push>
                                                {icon!(icondata::LuBell)} "Push Notifications"
                                            </CheckboxItem>
                                            <CheckboxItem checked=notifications
                                                .email>
                                                {icon!(icondata::LuMail)} "Email Notifications"
                                            </CheckboxItem>
                                        </MenuGroup>
                                    </MenuSubContent>
                                </MenuSub>
                            </MenuGroup>
                            <Separator />
                            <MenuGroup>
                                <MenuItem>
                                    {icon!(icondata::LuShield)} "Privacy & Security"
                                </MenuItem>
                            </MenuGroup>
                        </MenuSubContent>
                    </MenuSub>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem>{icon!(icondata::FiHelpCircle)} "Help & Support"</MenuItem>
                    <MenuItem>{icon!(icondata::LuFileText)} "Documentation"</MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem variant="destructive">
                        {icon!(icondata::LuLogOut)} "Sign Out"
                        <MenuShortcut>"⇧⌘Q"</MenuShortcut>
                    </MenuItem>
                </MenuGroup>
            </MenuContent>
        </DropdownMenu>
    }
}
