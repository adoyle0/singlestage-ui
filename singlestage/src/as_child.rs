use leptos::prelude::*;

use crate::patch_class::PatchClass;
#[component]
pub fn AsChild(#[prop(optional, into)] class:TextProp, children:Children) -> impl IntoView{
    view! { {children().add_any_attr(leptos::tachys::html::class::class(PatchClass(class)))} }
}