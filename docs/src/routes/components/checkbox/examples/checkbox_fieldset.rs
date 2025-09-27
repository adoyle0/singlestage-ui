use leptos::prelude::*;
use singlestage::{checkbox::*, Button, Label};

#[component]
pub fn CheckboxFieldsetExample() -> impl IntoView {
    view! {
        <form class="form flex flex-col gap-4">
            <header>
                <Label label_for="demo-form-checkboxes" class="text-base leading-normal">
                    "Sidebar"
                </Label>
                <p class="text-muted-foreground text-sm">
                    "Select the items you want to display in the sidebar."
                </p>
            </header>
            <fieldset id="demo-form-checkboxes" class="flex flex-col gap-2">
                <Label class="font-normal leading-tight">
                    <Checkbox name="demo-form-checkboxes" value="1" checked=RwSignal::new(true) />
                    "Recents"
                </Label>
                <Label class="font-normal leading-tight">
                    <Checkbox name="demo-form-checkboxes" value="2" checked=RwSignal::new(true) />
                    "Home"
                </Label>
                <Label class="font-normal leading-tight">
                    <Checkbox name="demo-form-checkboxes" value="3" />
                    "Applications"
                </Label>
                <Label class="font-normal leading-tight">
                    <Checkbox name="demo-form-checkboxes" value="4" />
                    "Desktop"
                </Label>
                <Label class="font-normal leading-tight">
                    <Checkbox name="demo-form-checkboxes" value="5" />
                    "Download"
                </Label>
                <Label class="font-normal leading-tight">
                    <Checkbox name="demo-form-checkboxes" value="6" />
                    "Documents"
                </Label>
            </fieldset>
            <Button class="w-fit">"Submit"</Button>
        </form>
    }
}
