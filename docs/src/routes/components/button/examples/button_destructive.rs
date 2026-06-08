use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonDestructiveExample() -> impl IntoView {
    view! { <Button variant="destructive">"Destructive"</Button> }
}
