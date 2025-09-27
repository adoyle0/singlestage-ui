use leptos::prelude::*;
use singlestage::card::*;

#[component]
pub fn CardAnatomy() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle />
                <CardDescription />
            </CardHeader>
            <CardContent />
            <CardFooter />
        </Card>
    }
}
