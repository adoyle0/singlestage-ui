use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownCheckboxesExample() -> impl IntoView {
    let show_status_bar = RwSignal::new(false);
    let show_activity_bar = RwSignal::new(false);
    let show_panel = RwSignal::new(false);

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">Open</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-40">
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"Appearance"</DropdownMenuLabel>
                    <DropdownMenuCheckboxItem checked=show_status_bar>
                        "Status Bar"
                    </DropdownMenuCheckboxItem>
                    <DropdownMenuCheckboxItem checked=show_activity_bar disabled=true>
                        "Activity Bar"
                    </DropdownMenuCheckboxItem>
                    <DropdownMenuCheckboxItem checked=show_panel>"Panel"</DropdownMenuCheckboxItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
