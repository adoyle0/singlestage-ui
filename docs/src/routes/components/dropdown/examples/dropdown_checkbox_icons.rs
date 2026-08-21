use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownCheckboxIconsExample() -> impl IntoView {
    let value = RwSignal::new(vec!["email".to_string(), "push".to_string()]);

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Notifications"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-48">
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"Notification Preferences"</DropdownMenuLabel>
                    <DropdownMenuCheckboxGroup value>
                        <DropdownMenuCheckboxItem value="email">
                            {icon!(icondata::LuMail)} "Email notifications"
                        </DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem value="sms">
                            {icon!(icondata::LuMessageSquare)} "SMS notifications"
                        </DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem value="push">
                            {icon!(icondata::LuBell)} "Push notifications"
                        </DropdownMenuCheckboxItem>
                    </DropdownMenuCheckboxGroup>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
