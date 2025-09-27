use leptos::prelude::*;
use singlestage::alert::*;

#[component]
pub fn AlertAnatomy() -> impl IntoView {
    view! {
        <Alert>
            <AlertTitle />
            <AlertDescription />
        </Alert>
    }
}
