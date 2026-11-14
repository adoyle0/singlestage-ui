use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TabsVerticalExample() -> impl IntoView {
    view! {
        <Tabs orientation="vertical">
            <TabsList>
                <TabsTrigger value="account">"Account"</TabsTrigger>
                <TabsTrigger value="password">"Password"</TabsTrigger>
                <TabsTrigger value="notifications">"Notifications"</TabsTrigger>
            </TabsList>
        </Tabs>
    }
}
