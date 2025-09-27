use leptos::prelude::*;
use singlestage::link::*;

#[component]
pub fn LinkExample() -> impl IntoView {
    view! { <Link href="#">"This is a link"</Link> }
}
