use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownCheckboxIconsExample() -> impl IntoView {
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
                <Button variant="outline">"Notifications"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-48">
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"Notification Preferences"</DropdownMenuLabel>
                    <CheckboxItem checked=notifications
                        .email>{icon!(icondata::LuMail)} "Email notifications"</CheckboxItem>
                    <CheckboxItem checked=notifications
                        .sms>{icon!(icondata::LuMessageSquare)} "SMS notifications"</CheckboxItem>
                    <CheckboxItem checked=notifications
                        .push>{icon!(icondata::LuBell)} "Push notifications"</CheckboxItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
