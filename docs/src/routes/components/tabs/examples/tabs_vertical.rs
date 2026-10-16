use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TabsVerticalExample() -> impl IntoView {
    view! {
        <Tabs value="account" orientation="vertical">
            <TabsList>
                <TabsTrigger value="account">"Account"</TabsTrigger>
                <TabsTrigger value="password">"Password"</TabsTrigger>
                <TabsTrigger value="notifications">"Notifications"</TabsTrigger>
            </TabsList>
        </Tabs>
    }
}
