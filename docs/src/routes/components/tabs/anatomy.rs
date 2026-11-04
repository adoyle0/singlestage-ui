use leptos::prelude::*;
use singlestage::*;

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
