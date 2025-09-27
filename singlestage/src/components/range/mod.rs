use leptos::prelude::*;

#[component]
pub fn Range(#[prop(optional, into)] autocomplete: MaybeProp<String>) -> impl IntoView {
    view! { "range" }
}
