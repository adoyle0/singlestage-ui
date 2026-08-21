use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownComplexExample() -> impl IntoView {
    let view = RwSignal::new(vec!["sidebar".to_string()]);
    let theme = RwSignal::new("light".to_string());
    let notifications = RwSignal::new(vec!["email".to_string(), "push".to_string()]);

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
                    <DropdownMenuCheckboxGroup value=view>
                        <DropdownMenuCheckboxItem value="sidebar">
                            {icon!(icondata::LuEye)} "Show Sidebar"
                        </DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem value="status_bar">
                            {icon!(icondata::FiLayout)} "Show Status Bar"
                        </DropdownMenuCheckboxItem>
                    </DropdownMenuCheckboxGroup>
                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>
                            {icon!(icondata::LuPalette)} "Theme"
                        </DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuGroup>
                                <DropdownMenuLabel>"Appearance"</DropdownMenuLabel>
                                <DropdownMenuRadioGroup value=theme>
                                    <DropdownMenuRadioItem value="light">
                                        {icon!(icondata::LuSun)} "Light"
                                    </DropdownMenuRadioItem>
                                    <DropdownMenuRadioItem value="dark">
                                        {icon!(icondata::LuMoon)} "Dark"
                                    </DropdownMenuRadioItem>
                                    <DropdownMenuRadioItem value="system">
                                        {icon!(icondata::LuMonitor)} "System"
                                    </DropdownMenuRadioItem>
                                </DropdownMenuRadioGroup>
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
                                            <DropdownMenuCheckboxGroup value=notifications>
                                                <DropdownMenuCheckboxItem value="push">
                                                    {icon!(icondata::LuBell)} "Push Notifications"
                                                </DropdownMenuCheckboxItem>
                                                <DropdownMenuCheckboxItem value="email">
                                                    {icon!(icondata::LuMail)} "Email Notifications"
                                                </DropdownMenuCheckboxItem>
                                            </DropdownMenuCheckboxGroup>
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
