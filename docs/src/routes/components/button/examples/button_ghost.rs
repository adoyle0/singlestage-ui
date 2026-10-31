use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGhostExample() -> impl IntoView {
    view! { <Button variant="ghost">"Ghost"</Button> }
}
