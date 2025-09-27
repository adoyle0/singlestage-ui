use leptos::prelude::*;
use singlestage::tabs::*;

#[component]
pub fn TabsAnatomy() -> impl IntoView {
    view! {
        <Tabs>
            <TabsList>
                <TabsTrigger />
            </TabsList>
            <TabsContent />
        </Tabs>
    }
}
