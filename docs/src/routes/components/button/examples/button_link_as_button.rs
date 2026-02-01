use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonLinkAsButtonExample() -> impl IntoView {
    view! {
        <Link render_as="button" href="/login">
            "Log in"
        </Link>
    }
}
