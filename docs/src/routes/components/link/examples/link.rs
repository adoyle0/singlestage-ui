use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn LinkExample() -> impl IntoView {
    view! { <Link href="#">"This is a link"</Link> }
}
