use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownCheckboxesExample() -> impl IntoView {
    let value: RwSignal<Vec<String>> = RwSignal::new(vec![]);

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">Open</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-40">
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"Appearance"</DropdownMenuLabel>
                    <DropdownMenuCheckboxGroup value>
                        <DropdownMenuCheckboxItem value="status_bar">
                            "Status Bar"
                        </DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem disabled=true value="activity_bar">
                            "Activity Bar"
                        </DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem value="panel">"Panel"</DropdownMenuCheckboxItem>
                    </DropdownMenuCheckboxGroup>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
