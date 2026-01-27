use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownComplexExample() -> impl IntoView {
    let theme = RwSignal::new("light".to_string());

    struct Notifications {
        email: RwSignal<bool>,
        sms: RwSignal<bool>,
        push: RwSignal<bool>,
    }

    let notifications = Notifications {
        email: RwSignal::new(true),
        sms: RwSignal::new(false),
        push: RwSignal::new(true),
    };

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Complex Menu"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-44">
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"File"</DropdownMenuLabel>
                    <DropdownMenuItem>
                        {icon!(icondata::LuFile)} "New File"
                        <DropdownMenuShortcut>"⌘N"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        {icon!(icondata::LuFolder)} "New Folder"
                        <DropdownMenuShortcut>"⇧⌘N"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>
                            {icon!(icondata::LuFolderOpen)} "Open Recent"
                        </DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuGroup>
                                <DropdownMenuLabel>"Recent Projects"</DropdownMenuLabel>
                                <DropdownMenuItem>
                                    {icon!(icondata::LuFileCode)} "Project Alpha"
                                </DropdownMenuItem>
                                <DropdownMenuItem>
                                    {icon!(icondata::LuFileCode)} "Project Beta"
                                </DropdownMenuItem>
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger>
                                        {icon!(icondata::FiMoreHorizontal)} "More Projects"
                                    </DropdownMenuSubTrigger>
                                    <DropdownMenuSubContent>
                                        <DropdownMenuItem>
                                            {icon!(icondata::LuFileCode)} "Project Gamma"
                                        </DropdownMenuItem>
                                        <DropdownMenuItem>
                                            {icon!(icondata::LuFileCode)} "Project Delta"
                                        </DropdownMenuItem>
                                    </DropdownMenuSubContent>
                                </DropdownMenuSub>
                            </DropdownMenuGroup>
                            <DropdownMenuSeparator />
                            <DropdownMenuGroup>
                                <DropdownMenuItem>
                                    {icon!(icondata::LuFolderSearch)} "Browse..."
                                </DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem>
                        {icon!(icondata::LuSave)} "Save"
                        <DropdownMenuShortcut>"⌘S"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        {icon!(icondata::LuDownload)} "Export"
                        <DropdownMenuShortcut>"⇧⌘E"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"View"</DropdownMenuLabel>
                    <Checkbox checked=notifications
                        .email>{icon!(icondata::LuEye)} "Show Sidebar"</Checkbox>
                    <Checkbox checked=notifications
                        .sms>{icon!(icondata::FiLayout)} "Show Status Bar"</Checkbox>
                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>
                            {icon!(icondata::LuPalette)} "Theme"
                        </DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuGroup>
                                <DropdownMenuLabel>"Appearance"</DropdownMenuLabel>
                                <RadioGroup value=theme>
                                    <Radio value="light">{icon!(icondata::LuSun)} "Light"</Radio>
                                    <Radio value="dark">{icon!(icondata::LuMoon)} "Dark"</Radio>
                                    <Radio value="system">
                                        {icon!(icondata::LuMonitor)} "System"
                                    </Radio>
                                </RadioGroup>
                            </DropdownMenuGroup>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"Account"</DropdownMenuLabel>
                    <DropdownMenuItem>
                        {icon!(icondata::LuUser)} "Profile"
                        <DropdownMenuShortcut>"⇧⌘P"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>{icon!(icondata::LuCreditCard)} "Billing"</DropdownMenuItem>
                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>
                            {icon!(icondata::LuSettings)} "Settings"
                        </DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuGroup>
                                <DropdownMenuLabel>"Preferences"</DropdownMenuLabel>
                                <DropdownMenuItem>
                                    {icon!(icondata::LuKeyboard)} "Keyboard Shortcuts"
                                </DropdownMenuItem>
                                <DropdownMenuItem>
                                    {icon!(icondata::LuLanguages)} "Language"
                                </DropdownMenuItem>
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger>
                                        {icon!(icondata::LuBell)} "Notifications"
                                    </DropdownMenuSubTrigger>
                                    <DropdownMenuSubContent>
                                        <DropdownMenuGroup>
                                            <DropdownMenuLabel>"Notification Types"</DropdownMenuLabel>
                                            <Checkbox checked=notifications
                                                .push>
                                                {icon!(icondata::LuBell)} "Push Notifications"
                                            </Checkbox>
                                            <Checkbox checked=notifications
                                                .email>
                                                {icon!(icondata::LuMail)} "Email Notifications"
                                            </Checkbox>
                                        </DropdownMenuGroup>
                                    </DropdownMenuSubContent>
                                </DropdownMenuSub>
                            </DropdownMenuGroup>
                            <DropdownMenuSeparator />
                            <DropdownMenuGroup>
                                <DropdownMenuItem>
                                    {icon!(icondata::LuShield)} "Privacy & Security"
                                </DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        {icon!(icondata::FiHelpCircle)} "Help & Support"
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        {icon!(icondata::LuFileText)} "Documentation"
                    </DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuItem variant="destructive">
                        {icon!(icondata::LuLogOut)} "Sign Out"
                        <DropdownMenuShortcut>"⇧⌘Q"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
