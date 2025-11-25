use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerCustomExample() -> impl IntoView {
    view! { <Spinner>{icon!(icondata::LuLoader, class="size-6 text-red-500")}</Spinner> }
}
