use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TabsLineExample() -> impl IntoView {
    view! {
        <Tabs value="overview">
            <TabsList variant="line">
                <TabsTrigger value="overview">"Overview"</TabsTrigger>
                <TabsTrigger value="analytics">"Analytics"</TabsTrigger>
                <TabsTrigger value="reports">"Reports"</TabsTrigger>
            </TabsList>
        </Tabs>
    }
}
