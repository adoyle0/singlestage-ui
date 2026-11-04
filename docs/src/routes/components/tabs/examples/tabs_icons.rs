use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TabsIconsExample() -> impl IntoView {
    view! {
        <Tabs>
            <TabsList>
                <TabsTrigger value="preview">{icon!(icondata::LuAppWindow)} "Preview"</TabsTrigger>
                <TabsTrigger value="code">{icon!(icondata::LuCode)} "Code"</TabsTrigger>
            </TabsList>
        </Tabs>
    }
}
