use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TabsDisabledExample() -> impl IntoView {
    view! {
        <Tabs value="home">
            <TabsList>
                <TabsTrigger value="home">"Home"</TabsTrigger>
                <TabsTrigger value="settings" disabled=true>
                    "Disabled"
                </TabsTrigger>
            </TabsList>
        </Tabs>
    }
}
