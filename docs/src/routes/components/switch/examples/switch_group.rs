use leptos::prelude::*;
use singlestage::{Button, CheckboxGroup, Switch};

#[component]
pub fn SwitchGroupExample() -> impl IntoView {
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

                <Switch value="recents">"Recents"</Switch>
                <Switch value="home">"Home"</Switch>
                <Switch value="applications">"Applications"</Switch>
                <Switch value="desktop">"Desktop"</Switch>
                <Switch value="download">"Download"</Switch>
                <Switch value="documents">"Documents"</Switch>
            </CheckboxGroup>

            <Button class="w-fit">"Submit"</Button>
        </form>
    }
}
