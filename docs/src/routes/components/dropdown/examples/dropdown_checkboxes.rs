use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownCheckboxesExample() -> impl IntoView {
    let show_status_bar = RwSignal::new(false);
    let show_activity_bar = RwSignal::new(false);
    let show_panel = RwSignal::new(false);

    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="outline">Open</Button>
            </Trigger>
            <MenuContent class="w-40">
                <MenuGroup>
                    <Label>"Appearance"</Label>
                    <CheckboxItem checked=show_status_bar>"Status Bar"</CheckboxItem>
                    <CheckboxItem checked=show_activity_bar disabled=true>
                        "Activity Bar"
                    </CheckboxItem>
                    <CheckboxItem checked=show_panel>"Panel"</CheckboxItem>
                </MenuGroup>
            </MenuContent>
        </DropdownMenu>
    }
}
