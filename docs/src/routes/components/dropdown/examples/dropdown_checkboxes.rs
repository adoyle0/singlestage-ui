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
                    <CheckboxItem checked=show_status_bar>"Status Bar"</CheckboxItem>
                    <CheckboxItem checked=show_activity_bar disabled=true>
                        "Activity Bar"
                    </CheckboxItem>
                    <CheckboxItem checked=show_panel>"Panel"</CheckboxItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
