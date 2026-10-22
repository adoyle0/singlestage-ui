use crate::Reactive;
use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub struct SheetContext {
    pub content_id: RwSignal<String>,
    pub open: Reactive<bool>,
    pub trigger_id: RwSignal<String>,
}

#[component]
pub fn Sheet(children: Children, #[prop(optional, into)] open: Reactive<bool>) -> impl IntoView {
    let content_id = RwSignal::new(String::new());
    let trigger_id = RwSignal::new(String::new());

    let context = SheetContext {
        content_id,
        open,
        trigger_id,
    };

    view! { <Provider value=context>{children()}</Provider> }
}
