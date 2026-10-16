use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CardAnatomy() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle />
                <CardDescription />
                <CardAction />
            </CardHeader>
            <CardContent />
            <CardFooter />
        </Card>
    }
}
