use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonLinkAsButtonExample() -> impl IntoView {
    view! {
        <Link as_button=true href="/login">
            "Log in"
        </Link>
    }
}
