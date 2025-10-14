use leptos::prelude::*;
use singlestage::{Button, checkbox::*};

#[component]
pub fn GroupExample() -> impl IntoView {
    let value = RwSignal::new(vec![
        "recents".to_string(),
        "home".to_string(),
        "applications".to_string(),
    ]);

    view! {
        <form class="form flex flex-col gap-4">
            <p>"Value: "{move || format!("{:#?}", value.get())}</p>

            <CheckboxGroup value>
                <legend>
                    "Sidebar"
                    <p class="text-(--muted-foreground) text-sm">
                        "Select the items you want to display in the sidebar."
                    </p>
                </legend>

                <Checkbox value="recents">"Recents"</Checkbox>
                <Checkbox value="home">"Home"</Checkbox>
                <Checkbox value="applications">"Applications"</Checkbox>
                <Checkbox value="desktop">"Desktop"</Checkbox>
                <Checkbox value="download">"Download"</Checkbox>
                <Checkbox value="documents">"Documents"</Checkbox>
            </CheckboxGroup>

            <Button class="w-fit">"Submit"</Button>
        </form>
    }
}
